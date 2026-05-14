//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 864/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk864<F: Float>(t1163: F, t4240: F, t3484: F, t3482: F, t3783: F, t394: F, t4210: F, t1446: F, t3908: F, t415: F, t1327: F, t3924: F, t4158: F, t1220: F, t13125: F, t13441: F, t14242: F, t14247: F, t14250: F, t14253: F, t14258: F, t3925: F, t3930: F, t412: F, sigma0: F) -> (F, F, F, F) {
    let t14260 = t4240 * t1163;
    let t14261 = t3484 * t14260;
    let t14262 = t3482 * t14261;
    let t14264 = t3783 * sigma0;
    let t14265 = t14264 * t394;
    let t14266 = t4210 * t1163;
    let t14267 = t14265 * t14266;
    let t14268 = t3482 * t14267;
    let t14270 = t3908 * t1446;
    let t14271 = t415 * t14270;
    let t14273 = t1327 * t3924;
    let t14274 = t14273 * t4158;
    let t14279 = -0.223494e0 * t3930 * t13441 + 0.223494e0 * t14242 * t3925 + 0.48640370370370370369e-1 * t14247 + t13125 * t412 + 0.44218518518518518518e-2 * t14250 + 0.72960555555555555553e-1 * t14253 + 0.55273148148148148145e-2 * t14258 - 0.11054629629629629629e-2 * t14262 + 0.99491666666666666664e-2 * t14268 - 0.19898333333333333333e-1 * t14271 + 0.223494e0 * t3930 * t14274 + 0.579e0 * t1220 * t14274;
    (t14262, t14268, t14271, t14279)
}
