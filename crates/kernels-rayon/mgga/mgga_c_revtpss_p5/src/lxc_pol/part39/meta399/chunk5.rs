//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1454/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1454(t3568: f64, t5486: f64, t1287: f64, t1794: f64, t3727: f64, t1770: f64, t3766: f64, t3759: f64, t5245: f64, t5457: f64, t5351: f64, t13126: f64, t487: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17917 = t5486 * t3568;
    let t17921 = t3727 * t1794 * t1287;
    let t17934 = t1770 * t3766;
    let t17941 = t3759 * t5245;
    let t17944 = t5457 * t3568;
    let t17945 = t5351 * t17944;
    let t17948 = t13126 * t487;
    (t17917, t17921, t17934, t17941, t17945, t17948)
}
