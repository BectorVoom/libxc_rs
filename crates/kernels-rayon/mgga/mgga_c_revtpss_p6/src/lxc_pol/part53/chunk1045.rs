//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1045/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1045(t32842: f64, t32853: f64, t32873: f64, t32883: f64, t3: f64, t2042: f64, t7696: f64, t2170: f64, t7331: f64, t7334: f64, t1461: f64, t32358: f64, t32360: f64, t32362: f64, t32365: f64, t32368: f64, t32371: f64, t32373: f64, t32377: f64, t573: f64, t8616: f64, t8771: f64, param_d: f64) -> (f64, f64, f64, f64) {
    let t32885 = t32842 + t32853 + t32873 + t32883;
    let t32886 = t3 * t32885;
    let t32897 = param_d * t32885;
    let t32901 = t7696 * t2042;
    let t32903 = t2170 * t7331;
    let t32905 = t2170 * t7334;
    let t32910 = 3.0_f64 * t1461 * t8771 + t32897 * t573 + 3.0_f64 * t32358 + 6.0_f64 * t32360 + 3.0_f64 * t32362 + t32365 + t32368 + t32371 + t32373 + t32377 + 3.0_f64 * t32901 + 6.0_f64 * t32903 + 3.0_f64 * t32905 + t8616;
    (t32885, t32886, t32897, t32910)
}
