//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 731/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk731(t36: f64, t5825: f64, t70: f64, t1470: f64, t1486: f64, t2275: f64, t5819: f64, t48: f64, t476: f64, t53: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5826 = t36 * t5825;
    let t5827 = t5826 * t70;
    let t5830 = t1470 * t1486;
    let t5835 = t2275 * t5819;
    let t5838 = t48 * t5825;
    let t5842 = 1.0_f64 / t53 / t476;
    let t5843 = sigma2 * t5842;
    (t5826, t5827, t5830, t5835, t5838, t5842, t5843)
}
