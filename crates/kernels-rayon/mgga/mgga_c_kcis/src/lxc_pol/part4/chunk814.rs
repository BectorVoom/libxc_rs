//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 814/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk814(t1745: f64, t330: f64, t1154: f64, t829: f64, t304: f64, t4920: f64, t1153: f64, t2429: f64, t3392: f64, t3394: f64, t3397: f64, t368: f64, t5130: f64, t5133: f64, t5135: f64, t5139: f64, t5143: f64, t5147: f64, t5151: f64, t86: f64) -> (f64, f64, f64, f64) {
    let t5153 = t1745 * t330;
    let t5155 = t1154 * t5153 * t829;
    let t5158 = t304 * t4920;
    let t5162 = t3392 - 0.17687407407407407407e-1_f64 * t3394 - 0.26531111111111111111e-1_f64 * t3397 - 0.17687407407407407407e-1_f64 * t5130 - 0.44218518518518518518e-1_f64 * t5133 * t5135 - 0.26531111111111111111e-1_f64 * t1153 * t5139 + 0.53062222222222222222e-1_f64 * t5133 * t5143 - 0.53062222222222222222e-1_f64 * t2429 * t5147 - 0.26531111111111111111e-1_f64 * t5151 - 0.26531111111111111111e-1_f64 * t1153 * t5155 - 0.39796666666666666666e-1_f64 * t86 * t368 * t5158;
    (t5153, t5155, t5158, t5162)
}
