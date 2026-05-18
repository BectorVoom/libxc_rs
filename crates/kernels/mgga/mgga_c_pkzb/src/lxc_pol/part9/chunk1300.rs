//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1300/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1300<F: Float>(t2234: F, t2240: F, t8198: F, t3069: F, t6201: F, t2198: F, t6199: F, t3073: F, t6193: F, t1184: F, t18589: F, t18592: F, t6143: F) -> (F, F, F, F) {
    let t22840 = F::new(0.48245938496077605201e2) * t2240 * t8198 * t2234;
    let t22841 = t3069 * t6201;
    let t22844 = F::new(0.1551780387578202009e4) * t6199 * t22841 * t2198;
    let t22847 = F::new(0.16081979498692535067e2) * t2240 * t3073 * t6193;
    let t22851 = F::new(0.24955700379505800916e5) * t18589 * t1184 * t18592 * t6143;
    (t22840, t22844, t22847, t22851)
}
