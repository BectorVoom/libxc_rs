//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 965/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk965<F: Float>(t13294: F, t14255: F, t3484: F, t5633: F, t1163: F, t4240: F, t3482: F, t3783: F, t394: F, t4210: F, t1446: F, t3908: F, sigma0: F) -> (F, F, F, F) {
    let t14256 = t14255 * t13294;
    let t14257 = t3484 * t14256;
    let t14258 = t5633 * t14257;
    let t14260 = t4240 * t1163;
    let t14261 = t3484 * t14260;
    let t14262 = t3482 * t14261;
    let t14264 = t3783 * sigma0;
    let t14265 = t14264 * t394;
    let t14266 = t4210 * t1163;
    let t14267 = t14265 * t14266;
    let t14268 = t3482 * t14267;
    let t14270 = t3908 * t1446;
    (t14258, t14262, t14268, t14270)
}
