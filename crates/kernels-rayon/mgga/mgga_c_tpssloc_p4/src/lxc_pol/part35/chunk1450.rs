//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1450/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1450(t104015: f64, t104282: f64, t104294: f64, t104296: f64, t104337: f64, t1737: f64, t2134: f64, t22012: f64, t22032: f64, t22185: f64, t24815: f64, t24821: f64, t27614: f64, t27617: f64, t27636: f64, t27637: f64, t27642: f64, t29644: f64, t29648: f64, t460: f64, t6203: f64, t6218: f64, t6221: f64, t7310: f64, t7320: f64, t7345: f64, t8040: f64, t95387: f64, t95512: f64, t95520: f64) -> f64 {
    let t109627 = t104294 / 384.0_f64 + t104296 / 384.0_f64 + t95512 / 432.0_f64 - 0.30279567070605293142e-3_f64 * t27636 * t27642 * t24821 * t6218 + 0.30279567070605293142e-3_f64 * t95387 * t29648 - 0.30279567070605293142e-3_f64 * t104282 * t8040 + 0.60559134141210586284e-3_f64 * t27636 * t27637 * t24815 * t6218 - 0.60559134141210586284e-3_f64 * t95387 * t29644 - 0.30279567070605293142e-3_f64 * t104337 + t95520 / 432.0_f64 - 0.10093189023535097714e-3_f64 * t2134 * t22032 * t460 * t7320 - 7.0_f64 / 648.0_f64 * t7310 * t22012 + 5.0_f64 / 2304.0_f64 * t27617 * t6203 + 5.0_f64 / 1152.0_f64 * t7345 * t22185 + t104015 * t1737 / 512.0_f64 + t27614 * t6221 / 512.0_f64;
    t109627
}
