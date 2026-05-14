//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1401/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1401<F: Float>(t26517: F, t21406: F, t21409: F, t22102: F, t22105: F, t22107: F, t22109: F, t26498: F, t26500: F, t26504: F, t26508: F, t26513: F, t26515: F, t5403: F, t7784: F, t2799: F, t5465: F) -> (F, F, F) {
    let t26518 = 0.33872559466666666666e-2 * t26517;
    let t26519 = 0.5143752e0 * t21406 - t21409 + 0.11407595979765752406e3 * t26498 - t22102 + t22105 + t22107 - 0.16265371950452609763e-1 * t26500 - 0.1714584e0 * t22109 + 0.10526802520742363173e2 * t26504 + 0.254044196e-2 * t26508 + t26513 - 0.31168546390226634766e3 * t26515 - t26518;
    let t26520 = t7784 * t5403;
    let t26522 = t2799 * t5465;
    (t26519, t26520, t26522)
}
