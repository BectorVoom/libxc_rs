//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3217/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3217(t55998: f64, t56034: f64, t56075: f64, t66935: f64, t1395: f64, t671: f64, t112: f64, t20148: f64, t12524: f64, t12813: f64, t1401: f64, t1458: f64, t16521: f64, t16524: f64, t16538: f64, t16541: f64, t19534: f64, t20162: f64, t20173: f64, t20176: f64, t2363: f64, t3938: f64, t3941: f64, t4072: f64, t5456: f64, t55568: f64, t55571: f64, t577: f64) -> (f64, f64) {
    let t66937 = t55998 + t56034 + t56075 + t66935;
    let t66940 = t1395 * t671;
    let t66958 = t20148 * t112;
    let t66961 = 108.0_f64 * t16524 * t16538 + 0.135e2_f64 * t1401 * t55568 + 27.0_f64 * t55571 * t5456 + 0.45e1_f64 * t66937 * t577 + 54.0_f64 * t66940 * t5456 + 54.0_f64 * t16521 * t4072 + 0.135e2_f64 * t20162 * t2363 + 54.0_f64 * t3941 * t1458 * t12813 + 27.0_f64 * t3938 * t19534 + 54.0_f64 * t16524 * t16541 + 108.0_f64 * t20173 * t20176 + 108.0_f64 * t12524 * t20176 + 27.0_f64 * t66958 * t671;
    (t66937, t66961)
}
