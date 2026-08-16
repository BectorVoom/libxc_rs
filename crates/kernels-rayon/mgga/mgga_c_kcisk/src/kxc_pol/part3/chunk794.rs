//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 794/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk794(t12261: f64, t2024: f64, t782: f64, t4419: f64, t5516: f64, t5510: f64, t2020: f64, t4597: f64, t12235: f64, t5006: f64, t10399: f64, t5486: f64) -> (f64, f64, f64, f64, f64) {
    let t12262 = t12261 * t2024;
    let t12263 = t782 * t12262;
    let t12265 = t4419 * t5516;
    let t12266 = t782 * t12265;
    let t12268 = t4419 * t5510;
    let t12269 = t782 * t12268;
    let t12271 = t2020 * t4597;
    let t12272 = t12271 * t12235;
    let t12273 = t5006 * t12272;
    let t12276 = t5486 * t10399;
    (t12263, t12266, t12269, t12273, t12276)
}
