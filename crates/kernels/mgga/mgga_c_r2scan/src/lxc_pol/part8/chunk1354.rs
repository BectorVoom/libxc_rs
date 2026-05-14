//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1354/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1354<F: Float>(t20097: F, t20107: F, t24725: F, t24733: F, t24735: F, t24742: F, t24922: F, t29128: F, t29130: F, t29146: F, t29152: F, t29155: F, t29158: F, t30364: F, t3192: F, t940: F) -> (F,) {
    let t33100 = -0.19043987679069580389e-1 * t29128 - 0.38087975358139160776e-1 * t29130 + 0.26004665220162805689e0 * t24922 * t3192 - 0.13002332610081402845e0 * t30364 * t940 + 0.3180600367820807838e-2 * t20097 + 0.73613752582167450608e0 * t20107 + 0.17465477326173296717e-1 * t29146 + 0.58903184358478742632e0 * t24725 - 0.69861909304693186866e-1 * t29152 - 0.1047928639570397803e0 * t29155 + 0.26023093918533882311e-2 * t29158 - t24733 - t24735 + 0.55488507004364032915e1 * t24742;
    (t33100,)
}
