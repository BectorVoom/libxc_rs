//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 868/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk868(t40339: f64, t40349: f64, t40351: f64, t40354: f64, t40356: f64, t40458: f64, t40479: f64, t40505: f64, t40560: f64, t40562: f64, t40578: f64, t275: f64, t9677: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t43385 = 0.11918087970123395032e-3_f64 * t40339;
    let t43390 = 0.39726959900411316772e-4_f64 * t40349;
    let t43391 = 0.11918087970123395032e-3_f64 * t40351;
    let t43392 = 0.11918087970123395032e-3_f64 * t40354;
    let t43393 = 0.39726959900411316772e-4_f64 * t40356;
    let t43422 = 0.15965655602485078085e0_f64 * t40458;
    let t43433 = 0.39726959900411316772e-4_f64 * t40479;
    let t43440 = 0.39726959900411316772e-4_f64 * t40505;
    let t43466 = 0.1489760996265424379e-3_f64 * t40560;
    let t43467 = 0.1489760996265424379e-3_f64 * t40562;
    let t43472 = 0.15965655602485078085e0_f64 * t40578;
    let t43481 = 2.0_f64 * t275 * t9677;
    (t43385, t43390, t43391, t43392, t43393, t43422, t43433, t43440, t43466, t43467, t43472, t43481)
}
