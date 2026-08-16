//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 443/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk443<F: Float>(t259: F, t116: F, t3391: F, t1111: F, t1118: F, t20: F, t918: F, t268: F, t1120: F, t272: F, t1123: F, t397: F, t3366: F) -> (F, F, F, F, F, F, F, F) {
    let t270 = F::cast_from(0.0_f64) < t259;
    let t3392 = t3391 * t116;
    let t3399 = t1111 * t1118;
    let t3405 = t918 * t20;
    let t3406 = t268 * t3405;
    let t3410 = F::cast_from(1.0_f64) / t1120 / t272;
    let t3411 = t1123 * t1123;
    let t3413 = t397 * t3410 * t3411;
    let t3417 = piecewise3::<F>(t270, t3366, -t3366);
    (t3392, t3399, t3405, t3406, t3410, t3411, t3413, t3417)
}
