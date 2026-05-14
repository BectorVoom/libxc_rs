//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1048/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1048<F: Float>(t10392: F, t106: F, t797: F, t97: F, t1039: F, t3250: F, t1044: F, t2995: F, t2266: F, t910: F, t9589: F, t8597: F, t2900: F, t990: F, t6621: F, t2358: F, t2904: F) -> (F, F, F, F, F, F, F, F) {
    let t10395 = t97 * t106 * t10392 * t797;
    let t10396 = t1039 * t3250;
    let t10397 = 3.0 * t10396;
    let t10399 = t2995 * t1044;
    let t10400 = 3.0 * t10399;
    let t10402 = t2266 * t9589 * t910;
    let t10403 = 9.0 * t10402;
    let t10407 = t2266 * t8597 * t910;
    let t10408 = 9.0 * t10407;
    let t10409 = t2900 * t990;
    let t10410 = t6621 * t10409;
    let t10413 = t2358 * t2904;
    (t10395, t10397, t10400, t10403, t10408, t10409, t10410, t10413)
}
