//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 796/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk796(t9296: f64, t9311: f64, t160: f64, t167: f64, t251: f64, t88: f64, t41: f64, t4879: f64, t4585: f64, t85: f64, t3250: f64, t2633: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9312 = t9296 + t9311;
    let t9323 = t167 * t160;
    let t9526 = t88 * t251;
    let t10138 = t4879 * t41;
    let t10269 = t85 * t4585;
    let t10338 = t85 * t3250 * t41;
    let t10443 = 6.0_f64 * t2633;
    (t9312, t9323, t9526, t10138, t10269, t10338, t10443)
}
