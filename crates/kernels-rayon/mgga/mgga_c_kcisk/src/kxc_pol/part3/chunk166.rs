//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 166/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk166(t340: f64, t639: f64, t642: f64, rho1: f64, sigma2: f64) -> (f64, f64, f64, f64, f64) {
    let t645 = 10.0_f64 / 9.0_f64 * t340 * t639 * t642;
    let t646 = t645 < -0.66725e-1_f64;
    let t648 = piecewise3(t646, 0.0_f64, 0.66725e-1_f64 + t645);
    let t649 = t648 * sigma2;
    let t650 = rho1 * rho1;
    let t651 = pow_1_3(rho1);
    let t652 = t651 * t651;
    let t654 = 1.0_f64 / t652 / t650;
    (t649, t650, t651, t654, t645)
}
