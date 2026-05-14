//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1377/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1377<F: Float>(t22219: F, t22222: F, t22227: F, t22229: F, t22233: F, t22239: F, t22242: F, t22246: F, t22249: F, t22250: F, t22255: F, t22260: F, t26638: F, t26642: F, t21787: F, t22264: F, t22267: F, t22270: F, t22278: F, t22285: F, t22288: F, t22292: F, t22296: F, t22305: F, t22308: F, t22312: F, t22315: F) -> (F, F) {
    let t33590 = 0.11407595979765752406e3 * t22219 + 0.57791679765211885293e1 * t22222 + t22227 + 0.1200612870296e-1 * t22229 + 0.600306435148e-2 * t22233 - t26638 - t22239 + t22242 - t22246 + t22249 + 0.26345324029629629628e-2 * t22250 + t22255 + t26642 + 0.64212977516902094772e0 * t22260;
    let t33594 = t22264 + t22267 + t22270 - t22278 + t22285 - 0.28518989949414381017e2 * t22288 - t22292 - t22296 + t22305 - t22308 - t22312 - t21787 + 0.10526802520742363173e2 * t22315;
    (t33590, t33594)
}
