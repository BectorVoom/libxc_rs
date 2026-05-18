//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 890/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk890<F: Float>(t1163: F, t4174: F, t3484: F, t3482: F, t1440: F, t3502: F, t1450: F, t1415: F, t1411: F, t3739: F, t3779: F, t1412: F) -> (F, F, F, F, F) {
    let t13316 = t4174 * t1163;
    let t13317 = t3484 * t13316;
    let t13318 = t3482 * t13317;
    let t13320 = t3502 * t1440;
    let t13321 = t1450 * t13320;
    let t13322 = t1415 * t13321;
    let t13323 = t1411 * t13322;
    let t13325 = t3739 * t3779;
    let t13327 = t1412 * t1412;
    (t13318, t13320, t13323, t13325, t13327)
}
