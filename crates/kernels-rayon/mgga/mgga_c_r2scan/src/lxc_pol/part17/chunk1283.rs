//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1283/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1283(t11325: f64, t12383: f64, t3275: f64, t11465: f64, t12422: f64, t3579: f64, t42263: f64, t12024: f64, t40713: f64, t38251: f64, t38259: f64, t38261: f64, t39107: f64, t39108: f64, t39109: f64, t39113: f64, t39114: f64, t39115: f64, t40642: f64, t42274: f64) -> (f64, f64, f64, f64, f64) {
    let t45044 = 5.0_f64 / 8.0_f64 * t3275 * t11325 * t12383;
    let t45046 = 5.0_f64 / 16.0_f64 * t12422 * t11465;
    let t45048 = t3579 * t42263 / 2.0_f64;
    let t45053 = 45.0_f64 / 32.0_f64 * t40713 * t12024;
    let t45054 = -t39107 + t39108 + t42274 - t39109 - 0.16163010989689081288e-5_f64 * t38251 - t45044 + t45046 + t45048 - 0.30487649791575028312e-3_f64 * t38259 + 0.30487649791575028312e-3_f64 * t38261 - t39113 - t39114 - t39115 + 0.12195059916630011325e-2_f64 * t40642 - t45053;
    (t45044, t45046, t45048, t45053, t45054)
}
