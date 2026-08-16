//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 402/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk402(t2561: f64, t2565: f64, t2569: f64, t2573: f64, t2577: f64, t2581: f64, t2588: f64, t2592: f64) -> f64 {
    let t2666 = 0.9375e-1_f64 * t2561 - 0.9375e-1_f64 * t2565 - 0.25e0_f64 * t2569 + 0.625e-1_f64 * t2573 - 0.101171875e-1_f64 * t2577 + 0.101171875e-1_f64 * t2581 + 0.53958333333333333333e-1_f64 * t2588 - 0.13489583333333333333e-1_f64 * t2592;
    t2666
}
