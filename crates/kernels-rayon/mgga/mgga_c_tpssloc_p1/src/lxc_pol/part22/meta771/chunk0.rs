//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2625/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2625(t1174: f64, t22059: f64, t3431: f64, t50846: f64, t63841: f64, t63843: f64, t63845: f64, t63886: f64, t63888: f64, t63893: f64, t63911: f64, t71333: f64, t71335: f64, t71337: f64, t71400: f64, t71406: f64, t71408: f64, t71411: f64, t71414: f64, t71417: f64, t71420: f64, t71423: f64, t71426: f64) -> (f64, f64) {
    let t73330 = t1174 * t3431 * t22059;
    let t73355 = t71333 / 18.0_f64 - t71335 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t71337 + 4.0_f64 / 27.0_f64 * t63841 + 2.0_f64 / 3.0_f64 * t63843 - t63845 / 9.0_f64 + t63886 / 3.0_f64 + 5.0_f64 / 27.0_f64 * t63888 - 10.0_f64 / 9.0_f64 * t63893 + 14.0_f64 / 81.0_f64 * t71400 - 5.0_f64 / 9.0_f64 * t63911 - t71406 / 6.0_f64 + t71408 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t71411 - 8.0_f64 / 9.0_f64 * t71414 + t71417 + 2.0_f64 * t71420 - 3.0_f64 * t71423 - 4.0_f64 * t71426 + 40.0_f64 / 27.0_f64 * t50846;
    (t73330, t73355)
}
