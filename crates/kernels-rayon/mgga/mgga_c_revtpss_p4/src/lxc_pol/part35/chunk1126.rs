//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1126/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1126(t25372: f64, t95536: f64, t2067: f64, t41117: f64, t25305: f64, t95540: f64, t10115: f64, t2063: f64, t10982: f64, t2061: f64, t9646: f64, t7058: f64, t95730: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95822 = t25372 * t95536;
    let t95862 = 0.81814717454467823679e-4_f64 * t41117 * t2067;
    let t95891 = 0.91399340044406952588e-2_f64 * t25305 * t95540;
    let t95893 = 0.11044544084478153697e-3_f64 * t10115 * t2063;
    let t95899 = 0.19637199382202157274e-3_f64 * t9646 * t2061 * t10982;
    let t95914 = 0.22487184191643109717e-1_f64 * t7058 * t95730;
    (t95822, t95862, t95891, t95893, t95899, t95914)
}
