//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2324/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2324(t184: f64, t20217: f64, t4194: f64, t607: f64, t13126: f64, t5398: f64, t16558: f64, t4195: f64, t16620: f64, t16693: f64, t16689: f64, t4202: f64) -> (f64, f64, f64, f64, f64) {
    let t67469 = t184 * t20217;
    let t67472 = 12.0_f64 * t4194 * t67469 * t607;
    let t67475 = 36.0_f64 * t4194 * t13126 * t5398;
    let t67478 = 36.0_f64 * t4194 * t4195 * t16558;
    let t67480 = 36.0_f64 * t16693 * t16620;
    let t67482 = 12.0_f64 * t16689 * t4202;
    (t67472, t67475, t67478, t67480, t67482)
}
