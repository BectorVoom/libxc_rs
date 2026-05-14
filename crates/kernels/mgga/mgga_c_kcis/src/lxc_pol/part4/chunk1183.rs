//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1183/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1183<F: Float>(t12530: F, t5916: F, t2051: F, t4303: F, t15860: F, t5909: F, t4260: F, t12265: F, t4291: F, t6012: F, t17331: F, t17335: F, t17337: F, t17339: F, t17342: F, t17344: F, t17347: F, t17350: F, t17353: F, t17355: F, t17358: F, t17360: F, t17362: F, t17364: F) -> (F, F, F, F, F) {
    let t17366 = t12530 * t5916;
    let t17368 = t2051 * t4303;
    let t17370 = t5909 * t15860;
    let t17371 = t4260 * t17370;
    let t17373 = t12265 * t4291;
    let t17374 = t17373 * t6012;
    let t17376 = -t17331 / 256.0 - t17335 / 48.0 + t17337 / 24.0 - 2.0 / 9.0 * t17339 - t17342 / 576.0 - t17344 / 8.0 - t17347 / 36.0 + t17350 / 576.0 + t17353 / 24.0 - t17355 / 16.0 + t17358 / 4.0 - t17360 / 16.0 + t17362 / 48.0 - t17364 / 12.0 + t17366 / 96.0 - t17368 / 576.0 + t17371 / 72.0 - t17374 / 64.0;
    (t17366, t17368, t17371, t17374, t17376)
}
