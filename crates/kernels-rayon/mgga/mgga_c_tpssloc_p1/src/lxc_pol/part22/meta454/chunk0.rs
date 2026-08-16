//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1819/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1819(t20118: f64, t20147: f64, t3: f64, t112: f64, t6470: f64, t576: f64, t671: f64, t1458: f64, t4072: f64, t5493: f64, t12524: f64, t1401: f64, t16521: f64, t16524: f64, t19534: f64, t3938: f64, t3941: f64, t5371: f64, t5376: f64, t5456: f64, t577: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20148 = t20118 + t20147;
    let t20149 = t3 * t20148;
    let t20162 = t6470 * t112;
    let t20173 = t576 * t671;
    let t20176 = t1458 * t4072;
    let t20181 = t5493 * t671;
    let t20186 = 0.45e1_f64 * t20148 * t577 + 0.135e2_f64 * t20162 * t671 + 27.0_f64 * t16521 * t1458 + 54.0_f64 * t16524 * t5376 + 27.0_f64 * t5371 * t4072 + 27.0_f64 * t12524 * t5456 + 27.0_f64 * t20173 * t5456 + 54.0_f64 * t3941 * t20176 + 0.135e2_f64 * t3938 * t5493 + 27.0_f64 * t3941 * t20181 + 0.135e2_f64 * t1401 * t19534;
    (t20148, t20149, t20162, t20173, t20176, t20181, t20186)
}
