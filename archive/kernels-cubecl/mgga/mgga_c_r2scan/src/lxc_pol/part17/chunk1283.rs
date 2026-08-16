//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1283/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1283<F: Float>(t11325: F, t12383: F, t3275: F, t11465: F, t12422: F, t3579: F, t42263: F, t12024: F, t40713: F, t38251: F, t38259: F, t38261: F, t39107: F, t39108: F, t39109: F, t39113: F, t39114: F, t39115: F, t40642: F, t42274: F) -> (F, F, F, F, F) {
    let t45044 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t3275 * t11325 * t12383;
    let t45046 = F::cast_from(5.0_f64) / F::cast_from(16.0_f64) * t12422 * t11465;
    let t45048 = t3579 * t42263 / F::cast_from(2.0_f64);
    let t45053 = F::cast_from(45.0_f64) / F::cast_from(32.0_f64) * t40713 * t12024;
    let t45054 = -t39107 + t39108 + t42274 - t39109 - F::cast_from(0.16163010989689081288e-5_f64) * t38251 - t45044 + t45046 + t45048 - F::cast_from(0.30487649791575028312e-3_f64) * t38259 + F::cast_from(0.30487649791575028312e-3_f64) * t38261 - t39113 - t39114 - t39115 + F::cast_from(0.12195059916630011325e-2_f64) * t40642 - t45053;
    (t45044, t45046, t45048, t45053, t45054)
}
