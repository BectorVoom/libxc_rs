//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1019/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1019<F: Float>(t524: F, t1576: F, t4370: F, t14164: F, t20: F, t533: F, t4502: F, t1572: F, t4416: F, t14866: F, t1589: F, t1586: F, t15006: F, t15009: F, t15011: F, t15014: F, t15041: F, t15044: F, t15047: F, t15052: F, t15053: F, t1583: F, t535: F, t541: F) -> F {
    let t536 = F::new(0.0) < t524;
    let t15056 = t4370 * t1576;
    let t15058 = t14164 * t20;
    let t15059 = t533 * t15058;
    let t15062 = t4502 * t1576;
    let t15064 = t1572 * t4416;
    let t15067 = piecewise3::<F>(t536, t14866, -t14866);
    let t15068 = t1589 * t15067;
    let t15069 = t1586 * t15068;
    let t15072 = -F::cast_from(0.59969295720591057378e-2_f64) * t15006 - F::cast_from(0.17990788716177317214e-1_f64) * t15009 + F::cast_from(0.26386490117060065246e0_f64) * t15011 * t1583 - F::cast_from(0.47975436576472845903e-1_f64) * t15014 + F::cast_from(0.2698618307426597582e-1_f64) * t15041 * t541 + F::cast_from(0.7915947035118019574e0_f64) * t15044 * t541 + F::cast_from(0.26386490117060065246e0_f64) * t15047 + t15052 - F::cast_from(0.21588946459412780656e0_f64) * t15053 * t541 + F::cast_from(0.2698618307426597582e-1_f64) * t15056 - F::cast_from(0.12313695387961363781e1_f64) * t15059 * t541 - F::cast_from(0.14392630972941853771e0_f64) * t15062 + F::cast_from(0.47975436576472845902e-1_f64) * t15064 - F::cast_from(0.2698618307426597582e-1_f64) * t535 * t15069;
    t15072
}
