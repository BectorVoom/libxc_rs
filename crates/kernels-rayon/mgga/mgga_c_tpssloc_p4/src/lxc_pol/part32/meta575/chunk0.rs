//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1950/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1950(t5371: f64, t7467: f64, t5456: f64, t576: f64, t1873: f64, t1458: f64, t3941: f64, t5493: f64, t1401: f64, t28017: f64, t1409: f64, t22510: f64, t24498: f64, t27356: f64, t5392: f64, t5398: f64, t5415: f64, t56: f64, t7251: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28892 = 27.0_f64 * t5371 * t7467;
    let t28893 = t576 * t5456;
    let t28895 = 27.0_f64 * t28893 * t1873;
    let t28896 = t7467 * t1458;
    let t28898 = 54.0_f64 * t3941 * t28896;
    let t28899 = t1873 * t5493;
    let t28901 = 27.0_f64 * t3941 * t28899;
    let t28903 = 0.135e2_f64 * t1401 * t28017;
    let t29473 = 88.0_f64 / 9.0_f64 * t5415 * t56 + 40.0_f64 / 9.0_f64 * t27356 * t1409 + 5.0_f64 / 18.0_f64 * t24498 * t5392 - 5.0_f64 / 6.0_f64 * t7251 * t5398 - t22510;
    (t28892, t28893, t28895, t28896, t28898, t28899, t28901, t28903, t29473)
}
