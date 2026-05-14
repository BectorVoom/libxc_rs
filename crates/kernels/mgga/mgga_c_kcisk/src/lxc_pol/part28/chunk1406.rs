//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1406/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1406<F: Float>(t34324: F, t34329: F, t733: F, t8831: F, t9709: F, t117369: F, t22995: F, t24457: F, t34368: F, t7299: F, t117410: F, t7333: F, t24112: F, t34321: F, t7440: F, t117426: F, t23000: F) -> (F, F, F, F, F, F, F, F, F) {
    let t122285 = t34329 * t34324;
    let t122288 = t733 * t8831;
    let t122289 = t122288 * t9709;
    let t122291 = t117369 * t22995;
    let t122293 = t34368 * t24457;
    let t122295 = t34329 * t7299;
    let t122297 = t117410 * t7333;
    let t122299 = t34321 * t24112;
    let t122301 = t117410 * t7440;
    let t122303 = t117426 * t23000;
    (t122285, t122289, t122291, t122293, t122295, t122297, t122299, t122301, t122303)
}
