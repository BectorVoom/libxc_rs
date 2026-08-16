//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 646/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk646(t10365: f64, t10366: f64, t5182: f64, t1894: f64, t3290: f64, t6675: f64, t5184: f64, t6674: f64, t5306: f64, t1802: f64, t1799: f64, t5074: f64, t5077: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10367 = t10365 * t10366;
    let t10368 = t5182 * t10367;
    let t10370 = t3290 * t1894;
    let t10371 = t6675 * t10370;
    let t10372 = t5184 * t10371;
    let t10373 = t6674 * t10372;
    let t10375 = t5306 * sigma2;
    let t10376 = t10375 * t1802;
    let t10377 = t1799 * t10376;
    let t10379 = t5074 * t5077;
    (t10368, t10370, t10373, t10375, t10377, t10379)
}
