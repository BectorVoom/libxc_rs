//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 103/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk103(t340: f64, t379: f64, t382: f64, sigma0: f64) -> (f64, f64) {
    let t385 = 10.0_f64 / 9.0_f64 * t340 * t379 * t382;
    let t386 = t385 < -0.66725e-1_f64;
    let t388 = piecewise3(t386, 0.0_f64, 0.66725e-1_f64 + t385);
    let t389 = t388 * sigma0;
    (t389, t385)
}
