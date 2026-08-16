//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1226/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1226(t5493: f64, t671: f64, t12524: f64, t1401: f64, t1458: f64, t16521: f64, t16524: f64, t19534: f64, t20148: f64, t20162: f64, t20173: f64, t20176: f64, t3938: f64, t3941: f64, t4072: f64, t5371: f64, t5376: f64, t5456: f64, t577: f64) -> f64 {
    let t20181 = t5493 * t671;
    let t20186 = 0.45e1_f64 * t20148 * t577 + 0.135e2_f64 * t20162 * t671 + 27.0_f64 * t16521 * t1458 + 54.0_f64 * t16524 * t5376 + 27.0_f64 * t5371 * t4072 + 27.0_f64 * t12524 * t5456 + 27.0_f64 * t20173 * t5456 + 54.0_f64 * t3941 * t20176 + 0.135e2_f64 * t3938 * t5493 + 27.0_f64 * t3941 * t20181 + 0.135e2_f64 * t1401 * t19534;
    t20186
}
