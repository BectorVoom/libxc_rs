//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1447/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1447(t22298: f64, t3032: f64, t104012: f64, t104107: f64, t104111: f64, t104120: f64, t104124: f64, t104190: f64, t104300: f64, t11721: f64, t1737: f64, t1748: f64, t22271: f64, t24729: f64, t27617: f64, t29644: f64, t29648: f64, t3508: f64, t475: f64, t6211: f64, t8040: f64, t86155: f64, t86157: f64, t86191: f64, t86208: f64, t86214: f64, t95295: f64, t95365: f64) -> f64 {
    let t109505 = t22298 * t3032;
    let t109528 = t104300 * t1748 / 72.0_f64 + 19.0_f64 / 288.0_f64 * t104012 * t1737 - 19.0_f64 / 432.0_f64 * t104107 * t1748 + 0.21801288290835811062e-1_f64 * t104190 * t8040 - 0.60559134141210586284e-3_f64 * t95295 * t29644 + 0.60559134141210586284e-3_f64 * t86155 * t86208 * t109505 * t11721 - 0.60559134141210586284e-3_f64 * t86155 * t86214 * t109505 * t3508 + 0.30279567070605293142e-3_f64 * t95295 * t29648 + 0.10093189023535097714e-3_f64 * t86155 * t86157 * t109505 * t475 - t95365 / 2304.0_f64 + t86191 + 0.48447307312968469026e-2_f64 * t104111 + 0.60559134141210586284e-3_f64 * t104120 - 0.30279567070605293142e-3_f64 * t104124 + t24729 * t22271 / 256.0_f64 - t27617 * t6211 / 384.0_f64;
    t109528
}
