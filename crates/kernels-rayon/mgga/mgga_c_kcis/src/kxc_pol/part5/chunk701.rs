//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 701/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk701(t1153: f64, t2429: f64, t3392: f64, t3394: f64, t3397: f64, t368: f64, t5130: f64, t5133: f64, t5135: f64, t5139: f64, t5143: f64, t5147: f64, t5151: f64, t5155: f64, t5158: f64, t86: f64) -> f64 {
    let t5162 = t3392 - 0.17687407407407407407e-1_f64 * t3394 - 0.26531111111111111111e-1_f64 * t3397 - 0.17687407407407407407e-1_f64 * t5130 - 0.44218518518518518518e-1_f64 * t5133 * t5135 - 0.26531111111111111111e-1_f64 * t1153 * t5139 + 0.53062222222222222222e-1_f64 * t5133 * t5143 - 0.53062222222222222222e-1_f64 * t2429 * t5147 - 0.26531111111111111111e-1_f64 * t5151 - 0.26531111111111111111e-1_f64 * t1153 * t5155 - 0.39796666666666666666e-1_f64 * t86 * t368 * t5158;
    t5162
}
