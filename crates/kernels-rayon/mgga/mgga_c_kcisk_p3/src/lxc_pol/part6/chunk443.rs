//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 443/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk443(t259: f64, t116: f64, t3391: f64, t1111: f64, t1118: f64, t20: f64, t918: f64, t268: f64, t1120: f64, t272: f64, t1123: f64, t397: f64, t3366: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t270 = 0.0_f64 < t259;
    let t3392 = t3391 * t116;
    let t3399 = t1111 * t1118;
    let t3405 = t918 * t20;
    let t3406 = t268 * t3405;
    let t3410 = 1.0_f64 / t1120 / t272;
    let t3411 = t1123 * t1123;
    let t3413 = t397 * t3410 * t3411;
    let t3417 = piecewise3(t270, t3366, -t3366);
    (t3392, t3399, t3405, t3406, t3410, t3411, t3413, t3417)
}
