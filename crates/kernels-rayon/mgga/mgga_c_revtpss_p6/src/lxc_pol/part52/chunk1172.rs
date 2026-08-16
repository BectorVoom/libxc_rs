//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1172/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1172(t121024: f64, t33964: f64, t125668: f64, t32206: f64, t32211: f64, t3936: f64, t121057: f64, t33922: f64, t32283: f64, t33943: f64, t1401: f64, t32194: f64, t32195: f64, t5659: f64, t5673: f64) -> (f64, f64, f64, f64, f64) {
    let t125775 = t121024 * t33964;
    let t125780 = t32206 * t3936 * t32211 * t125668;
    let t125782 = t121057 * t33922;
    let t125784 = t33943 * t32283;
    let t125785 = t125784 * t1401;
    let t125793 = t32194 * t5673 * t32195 * t5659;
    (t125775, t125780, t125782, t125785, t125793)
}
