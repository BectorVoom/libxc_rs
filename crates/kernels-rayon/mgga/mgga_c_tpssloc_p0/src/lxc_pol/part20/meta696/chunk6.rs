//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2660/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2660(t1307: f64, t16153: f64, t12300: f64, t5289: f64, t16208: f64, t3799: f64, t39249: f64, t39256: f64, t39261: f64, t39266: f64, t39304: f64, t39309: f64, t39312: f64, t39316: f64, t39320: f64, t53778: f64, t53780: f64, t53783: f64, t53788: f64, t53797: f64, t53799: f64, t53800: f64) -> (f64, f64, f64, f64) {
    let t54284 = t16153 * t1307;
    let t54293 = t12300 * t5289;
    let t54295 = t3799 * t16208;
    let t54311 = -t53778 - t53780 + t53783 + t53788 - t39249 - t39256 + t53797 - t53799 - t39261 - t39266 - t39304 + t53800 - t39309 + t39312 + t39316 + t39320;
    (t54284, t54293, t54295, t54311)
}
