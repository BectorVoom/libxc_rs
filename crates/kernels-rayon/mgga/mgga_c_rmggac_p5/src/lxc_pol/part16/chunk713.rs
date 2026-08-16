//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 713/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk713(t10015: f64, t10019: f64, t10025: f64, t10031: f64, t10033: f64, t2474: f64, t534: f64, t72: f64, t10041: f64, t10046: f64, t10051: f64, t10054: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10362 = 0.1702583995731913576e-4_f64 * t10015;
    let t10363 = 0.85129199786595678799e-5_f64 * t10019;
    let t10364 = 0.5107751987195740728e-4_f64 * t10025;
    let t10365 = 0.1702583995731913576e-4_f64 * t10031;
    let t10366 = 0.1702583995731913576e-4_f64 * t10033;
    let t10367 = t534 * t2474;
    let t10368 = t72 * t10367;
    let t10369 = 2.0_f64 * t10368;
    let t10370 = 0.1702583995731913576e-4_f64 * t10041;
    let t10371 = 0.85129199786595678799e-5_f64 * t10046;
    let t10374 = 0.23942587439980034662e-4_f64 * t10051;
    let t10375 = 0.35922725105591425692e0_f64 * t10054;
    (t10362, t10363, t10364, t10365, t10366, t10367, t10369, t10370, t10371, t10374, t10375)
}
