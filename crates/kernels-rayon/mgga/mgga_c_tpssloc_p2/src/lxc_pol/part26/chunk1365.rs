//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1365/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1365(t11713: f64, t11717: f64, t24727: f64, t11708: f64, t24732: f64, t7337: f64, t11651: f64, t24733: f64, t11797: f64, t7345: f64, t11724: f64, t11731: f64, t11741: f64, t11781: f64, t24664: f64, t24670: f64, t24706: f64, t3518: f64, t475: f64, t7316: f64, t86146: f64, t86149: f64, t86155: f64, t86157: f64, t86158: f64) -> f64 {
    let t86164 = t11713 * t24727 * t11717;
    let t86167 = t11708 * t24732;
    let t86171 = t11713 * t7337 * t11717;
    let t86174 = t24733 * t11651;
    let t86176 = t7345 * t11797;
    let t86182 = t86146 * t11724 / 256.0_f64 - 0.60559134141210586284e-3_f64 * t86149 * t24664 + 0.30279567070605293142e-3_f64 * t86149 * t24670 + 0.10093189023535097714e-3_f64 * t86155 * t86157 * t86158 * t475 - t86164 * t11731 / 256.0_f64 - t86167 * t3518 / 512.0_f64 + t86171 * t11741 / 1536.0_f64 - t86174 / 768.0_f64 - t86176 / 1152.0_f64 - 5.0_f64 / 2592.0_f64 * t7345 * t11781 + 0.30279567070605293142e-3_f64 * t7316 * t24706;
    t86182
}
