//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 481/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk481(t1065: f64, t159: f64, t631: f64, t2297: f64, t910: f64, t914: f64, t287: f64, t913: f64, t275: f64, t273: f64, t276: f64, t2846: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2850 = t159 * t1065;
    let t2851 = t631 * t631;
    let t2852 = 1.0_f64 / t2851;
    let t2857 = 1.0_f64 / t2297;
    let t2869 = t910 * t914;
    let t2872 = t913 * t287;
    let t2873 = 1.0_f64 / t2872;
    let t2874 = t275 * t2873;
    let t2880 = 1.0_f64 / t276 / t273;
    let t2884 = 4.0_f64 / 9.0_f64 * t2846;
    (t2850, t2852, t2857, t2869, t2874, t2880, t2884)
}
