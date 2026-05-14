//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1251/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1251<F: Float>(t1275: F, t2376: F, t1004: F, t6660: F, t1276: F, t1277: F, t1289: F, t19141: F, t19182: F, t23415: F, t23448: F, t23492: F, t2381: F, t2391: F, t313: F, t321: F, t6654: F, t6661: F, t6662: F, t6665: F, t819: F, t826: F, t8358: F, t8370: F, t8373: F, t8395: F) -> (F,) {
    let t23495 = t2376 * t1275;
    let t23498 = t1004 * t6660;
    let t23518 = -t819 * (3.0 / 10.0 * t313 * t23415 + t19182) + (t23448 + t23492) * t321 + 6.0 * t23495 * t1277 - 6.0 * t23498 * t6662 + 6.0 * t8358 * t6665 + 6.0 * t19141 * t2381 + 12.0 * t6654 * t8370 + 6.0 * t6654 * t8373 - 18.0 * t6661 * t2391 * t1277 + 6.0 * t1276 * t8395 * t826 + 6.0 * t1276 * t2391 * t1289;
    (t23518,)
}
