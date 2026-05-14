//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1185/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1185<F: Float>(t3069: F, t6201: F, t2198: F, t6199: F, t2240: F, t3073: F, t6193: F, t1184: F, t18589: F, t18592: F, t6143: F, t18854: F, t2252: F, t2259: F, t22795: F, t22815: F, t22822: F, t22825: F, t22826: F, t22829: F, t22837: F, t22840: F, t3103: F, t6269: F, t6272: F, t6303: F, t6314: F, t8068: F, t8107: F, t8132: F, t8135: F, t863: F, t871: F) -> (F, F, F, F) {
    let t22841 = t3069 * t6201;
    let t22844 = 0.1551780387578202009e4 * t6199 * t22841 * t2198;
    let t22847 = 0.16081979498692535067e2 * t2240 * t3073 * t6193;
    let t22851 = 0.24955700379505800916e5 * t18589 * t1184 * t18592 * t6143;
    let t22856 = 3.0 * t6303 * t3103 + 3.0 * t2252 * t8068 + 1.0 * t863 * (t22795 + t22815) * t871 + t22822 + t22825 - 6.0 * t22826 * t2259 - 0.19298375398431042081e3 * t22829 * t6314 + 0.35089341735807877242e1 * t8107 * t6269 - t22837 - t22840 - t22844 - t22847 - t22851 - 6.0 * t6272 * t8132 - 0.57895126195293126242e3 * t18854 * t8135;
    (t22844, t22847, t22851, t22856)
}
