//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1355/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1355(t27370: f64, t28342: f64, t5737: f64, t1386: f64, t7080: f64, t3717: f64, t1385: f64, t28372: f64, t101965: f64, t101969: f64, t101980: f64, t1380: f64, t28443: f64, t29284: f64, t52696: f64, t7908: f64, t94208: f64, t98058: f64, t98074: f64, t98105: f64, t98155: f64) -> (f64, f64, f64, f64) {
    let t103191 = t27370 * t28342 * t5737;
    let t103199 = t1386 * t7080;
    let t103204 = t3717 * t7080;
    let t103206 = t28372 * t103204 * t1385;
    let t103209 = 0.49745833333333333332e-2_f64 * t101965 - 0.16581944444444444444e-2_f64 * t101969 - t98058 - 0.13901041666666666667e-2_f64 * t7908 * t103191 + t98074 - 0.16489724537037037037e-3_f64 * t98155 * t28443 + 0.61836467013888888889e-4_f64 * t94208 * t29284 - t98105 + 0.16581944444444444444e-2_f64 * t101980 + 0.41703125000000000001e-2_f64 * t7908 * t52696 * t103199 * t1380 + 0.13901041666666666667e-2_f64 * t7908 * t103206;
    (t103191, t103199, t103206, t103209)
}
