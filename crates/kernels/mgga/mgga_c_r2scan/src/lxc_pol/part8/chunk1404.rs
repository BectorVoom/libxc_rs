//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1404/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1404<F: Float>(t10184: F, t20792: F, t2169: F, t22850: F, t25633: F, t2612: F, t29948: F, t29953: F, t29960: F, t29964: F, t29966: F, t29993: F, t29998: F, t30001: F, t3053: F, t32266: F, t32787: F, t506: F, t5108: F, t5109: F, t529: F, t551: F, t552: F, t566: F) -> (F,) {
    let t34067 = 0.29272321618148349056e-1 * t29948 + 0.58544643236296698111e-1 * t29953 - 0.51410067763503603056e-4 * t20792 + t25633 - 0.13002332610081402845e0 * t2169 * t10184 - 0.13002332610081402845e0 * t566 * t551 * t552 * t32266 - 0.41607464352260489103e1 * t29960 + 0.10401866088065122276e1 * t29964 + 0.20803732176130244552e1 * t29966 + 0.32927245914677557994e1 * t22850 * t529 * t506 * t32787 - 0.39006997830244208535e0 * t5108 * t5109 * t3053 * t2612 - 0.17465477326173296717e-1 * t29993 + 0.1047928639570397803e0 * t29998 + 0.52396431978519890151e-1 * t30001;
    (t34067,)
}
