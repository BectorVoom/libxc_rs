//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 873/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk873(t10355: f64, t22688: f64, t4201: f64, t5825: f64, t22671: f64, t48: f64, t477: f64, t53: f64, t10368: f64, t4210: f64, t60: f64, t10379: f64, t1480: f64, t1483: f64, t44: f64, t56: f64, t5843: f64, t5848: f64, t5851: f64, t61: f64, sigma2: f64) -> (f64, f64) {
    let t22689 = t10355 * t22688;
    let t22692 = t4201 * t5825;
    let t22695 = t48 * t22671;
    let t22699 = 1.0_f64 / t53 / t477;
    let t22700 = sigma2 * t22699;
    let t22709 = t10368 * t22688;
    let t22712 = t4210 * t5825;
    let t22715 = t60 * t22671;
    let t22718 = -5.0_f64 / 108.0_f64 * t44 * t22689 + 5.0_f64 / 6.0_f64 * t44 * t22692 + 5.0_f64 / 6.0_f64 * t44 * t22695 - 1232.0_f64 / 27.0_f64 * t22700 * t61 - 220.0_f64 / 9.0_f64 * t5843 * t1483 - 20.0_f64 / 9.0_f64 * t1480 * t5848 + 20.0_f64 / 3.0_f64 * t1480 * t5851 + 5.0_f64 / 108.0_f64 * t56 * t22709 + 5.0_f64 / 6.0_f64 * t56 * t22712 - 5.0_f64 / 6.0_f64 * t56 * t22715 + t10379;
    (t22700, t22718)
}
