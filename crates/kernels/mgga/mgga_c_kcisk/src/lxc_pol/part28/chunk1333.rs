//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1333/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1333<F: Float>(t1907: F, t34286: F, t5217: F, t9963: F, t116126: F, t116129: F, t116133: F, t34593: F, t9739: F, t116149: F, t17182: F, t34399: F, t33196: F, t33167: F, t34435: F, t1310: F, t5508: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t117563 = t34286 * t1907;
    let t117580 = t9963 * t5217;
    let t117613 = 0.15476481481481481481e-2 * t116126;
    let t117616 = 0.10317654320987654321e-2 * t116129;
    let t117618 = 0.30952962962962962962e-2 * t116133;
    let t117621 = t34593 * t9739;
    let t117629 = 0.15476481481481481481e-2 * t116149;
    let t117633 = t17182 * t34399;
    let t117635 = 0.13402777777777777778e-2 * t33196 * t117633;
    let t117639 = 0.11574074074074074074e-2 * t34435 * t33167;
    let t117652 = t1310 * t5508;
    (t117563, t117580, t117613, t117616, t117618, t117621, t117629, t117633, t117635, t117639, t117652)
}
