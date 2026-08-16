//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2702/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2702(t12466: f64, t12477: f64, t1297: f64, t1390: f64, t193: f64, t39249: f64, t39256: f64, t39261: f64, t39266: f64, t39304: f64, t39309: f64, t39312: f64, t39316: f64, t5126: f64, t5161: f64, t5308: f64, t533: f64, t53778: f64, t53780: f64, t53783: f64, t53788: f64, t53789: f64, t53797: f64, t53799: f64, t53800: f64, t53856: f64, t54832: f64, t55088: f64, t55124: f64, t55155: f64) -> f64 {
    let t55161 = -t53778 - t53780 + t53783 - 18.0_f64 * t5126 * t12477 * t5308 + t53788 - t39249 - 18.0_f64 * t5126 * t5161 * t53789 + 18.0_f64 * t5126 * t12466 * t5308 - t39256 + t53797 - t53799 - t39261 - t39266 - t39304 + t53800 + 3.0_f64 * t193 * t1297 * t53856 + t193 * t533 * (t54832 + t55088 + t55124 + t55155) * t1390 - t39309 + t39312 + t39316;
    t55161
}
