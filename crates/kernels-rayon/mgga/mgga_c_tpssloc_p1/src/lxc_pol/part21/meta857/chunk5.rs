//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3113/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3113(t300: f64, t63457: f64, t63506: f64, t63561: f64, t63611: f64, t63715: f64, t63760: f64, t64260: f64, t64442: f64, t1254: f64, t5091: f64, t11282: f64, t6084: f64) -> (f64, f64, f64) {
    let t64446 = t300 * (t63457 + t63506 + t63561 + t63611 + t63715 + t63760 + t64260 + t64442);
    let t64447 = t1254 * t5091;
    let t64451 = t11282 * t6084;
    (t64446, t64447, t64451)
}
