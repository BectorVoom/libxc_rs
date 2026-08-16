//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1355/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1355<F: Float>(t27370: F, t28342: F, t5737: F, t1386: F, t7080: F, t3717: F, t1385: F, t28372: F, t101965: F, t101969: F, t101980: F, t1380: F, t28443: F, t29284: F, t52696: F, t7908: F, t94208: F, t98058: F, t98074: F, t98105: F, t98155: F) -> (F, F, F, F) {
    let t103191 = t27370 * t28342 * t5737;
    let t103199 = t1386 * t7080;
    let t103204 = t3717 * t7080;
    let t103206 = t28372 * t103204 * t1385;
    let t103209 = F::cast_from(0.49745833333333333332e-2_f64) * t101965 - F::cast_from(0.16581944444444444444e-2_f64) * t101969 - t98058 - F::cast_from(0.13901041666666666667e-2_f64) * t7908 * t103191 + t98074 - F::cast_from(0.16489724537037037037e-3_f64) * t98155 * t28443 + F::cast_from(0.61836467013888888889e-4_f64) * t94208 * t29284 - t98105 + F::cast_from(0.16581944444444444444e-2_f64) * t101980 + F::cast_from(0.41703125000000000001e-2_f64) * t7908 * t52696 * t103199 * t1380 + F::cast_from(0.13901041666666666667e-2_f64) * t7908 * t103206;
    (t103191, t103199, t103206, t103209)
}
