//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 707/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk707<F: Float>(t2148: F, t6166: F, t6165: F, t1600: F, t1629: F, t2078: F, t537: F, t255: F, t571: F, t122: F, t2111: F, t409: F, t57: F, t128: F, t494: F, t538: F) -> (F, F, F, F, F, F, F, F) {
    let t6167 = t2148 * t6166;
    let t6168 = t6165 * t6167;
    let t6178 = t1600 * t1629;
    let t6180 = t537 * t2078;
    let t6182 = t571 * t6180 * t255;
    let t6188 = t2111 * t122;
    let t6189 = t409 * t57;
    let t6190 = t6189 * t128;
    let t6191 = t6188 * t6190;
    let t6192 = t538 * t494;
    (t6168, t6178, t6182, t6188, t6189, t6190, t6191, t6192)
}
