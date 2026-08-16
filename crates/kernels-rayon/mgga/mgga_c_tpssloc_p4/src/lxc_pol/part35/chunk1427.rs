//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1427/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1427(t22574: f64, t28830: f64, t33136: f64, t106956: f64, t1874: f64, t1873: f64, t20347: f64, t3941: f64, t1458: f64, t28017: f64, t5493: f64, t7467: f64) -> (f64, f64, f64, f64, f64) {
    let t107533 = 18.0_f64 * t22574 * t33136 * t28830;
    let t107539 = 6.0_f64 * t106956 * t1874;
    let t107552 = 27.0_f64 * t3941 * t1873 * t20347;
    let t107555 = 81.0_f64 * t3941 * t28017 * t1458;
    let t107558 = 81.0_f64 * t3941 * t7467 * t5493;
    (t107533, t107539, t107552, t107555, t107558)
}
