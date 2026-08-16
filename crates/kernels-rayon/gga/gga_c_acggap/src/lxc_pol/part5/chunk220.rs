//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 220/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk220(t729: f64, t747: f64, t31: f64, t4: f64, t668: f64, t132: f64, t200: f64, t220: f64, t721: f64, t199: f64, t27: f64, t13: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t748 = t729 * t747;
    let t752 = t4 * t668 * t31;
    let t753 = 0.14764627977777777777e-2_f64 * t752;
    let t754 = t132 * t200;
    let t756 = t721 * t754 * t220;
    let t757 = 0.35616666666666666666e-1_f64 * t756;
    let t758 = t199 * t27;
    let t759 = 1.0_f64 / t758;
    let t760 = t13 * t759;
    (t748, t753, t754, t757, t758, t759, t760)
}
