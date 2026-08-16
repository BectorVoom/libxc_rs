//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 201/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk201(t207: f64, t215: f64, t782: f64, t154: f64, t229: f64, t205: f64, t210: f64, t214: f64, t776: f64, t16: f64, t59: f64, t120: f64, t212: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t785 = 0.19444444444444444444e-2_f64 * t782 * t207 * t215;
    let t786 = t154 * t229;
    let t787 = t205 * t786;
    let t789 = t210 * t214 * t776;
    let t792 = t59 * t16;
    let t794 = t120 * t212;
    (t785, t786, t787, t789, t792, t794)
}
