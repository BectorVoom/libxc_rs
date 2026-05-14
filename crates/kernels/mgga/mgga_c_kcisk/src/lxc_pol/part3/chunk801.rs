//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 801/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk801<F: Float>(t1163: F, t4174: F, t3484: F, t3482: F, t1440: F, t3502: F, t1450: F, t1415: F, t1411: F, t3739: F, t3779: F, t1412: F, t453: F, t3786: F, t1341: F, t3764: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13316 = t4174 * t1163;
    let t13317 = t3484 * t13316;
    let t13318 = t3482 * t13317;
    let t13320 = t3502 * t1440;
    let t13321 = t1450 * t13320;
    let t13322 = t1415 * t13321;
    let t13323 = t1411 * t13322;
    let t13325 = t3739 * t3779;
    let t13327 = t1412 * t1412;
    let t13328 = 1.0 / t13327;
    let t13329 = t453 * t13328;
    let t13330 = t13329 * sigma0;
    let t13331 = t3786 * t1440;
    let t13332 = t1341 * t13331;
    let t13333 = t13330 * t13332;
    let t13334 = t1411 * t13333;
    let t13336 = t3764 * t3786;
    (t13318, t13320, t13323, t13325, t13328, t13329, t13331, t13334, t13336)
}
