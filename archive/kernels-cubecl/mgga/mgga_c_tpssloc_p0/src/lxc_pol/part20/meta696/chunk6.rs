//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2660/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2660<F: Float>(t1307: F, t16153: F, t12300: F, t5289: F, t16208: F, t3799: F, t39249: F, t39256: F, t39261: F, t39266: F, t39304: F, t39309: F, t39312: F, t39316: F, t39320: F, t53778: F, t53780: F, t53783: F, t53788: F, t53797: F, t53799: F, t53800: F) -> (F, F, F, F) {
    let t54284 = t16153 * t1307;
    let t54293 = t12300 * t5289;
    let t54295 = t3799 * t16208;
    let t54311 = -t53778 - t53780 + t53783 + t53788 - t39249 - t39256 + t53797 - t53799 - t39261 - t39266 - t39304 + t53800 - t39309 + t39312 + t39316 + t39320;
    (t54284, t54293, t54295, t54311)
}
