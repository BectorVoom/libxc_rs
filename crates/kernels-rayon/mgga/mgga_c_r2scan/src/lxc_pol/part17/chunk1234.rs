//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1234/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1234(t38597: f64, t39943: f64, t39950: f64, t41623: f64, t41624: f64, t43424: f64, t43426: f64, t43428: f64, t43432: f64, t43435: f64, t43438: f64, t43441: f64) -> f64 {
    let t44388 = -t41623 - 0.17336443480108537126e0_f64 * t43424 - 0.17336443480108537126e0_f64 * t43426 + t41624 + 0.12805040077930161442e0_f64 * t43428 + t39943 - t38597 + t39950 + 0.54878743191129263322e-2_f64 * t43432 - 0.46230515946956099003e0_f64 * t43435 - 0.27738309568173659403e1_f64 * t43438 + 0.93149212406257582492e-1_f64 * t43441;
    t44388
}
