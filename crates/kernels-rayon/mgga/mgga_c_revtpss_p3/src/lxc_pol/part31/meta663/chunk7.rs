//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2252/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2252(t105818: f64, t105822: f64, t105826: f64, t105830: f64, t105834: f64, t105837: f64, t105839: f64, t105841: f64, t105843: f64, t109278: f64, t109282: f64, t109288: f64, t1461: f64, t2040: f64, t22556: f64, t22568: f64, t30171: f64, t573: f64, t5805: f64, t6945: f64, t7324: f64, t7944: f64, param_d: f64) -> f64 {
    let t109289 = t109278 * t573 * param_d + 3.0_f64 * t1461 * t30171 + 6.0_f64 * t2040 * t22556 + 3.0_f64 * t2040 * t22568 + 6.0_f64 * t5805 * t7944 + 6.0_f64 * t6945 * t7324 + t105818 + t105822 + t105826 + t105830 + t105834 + t105837 + t105839 + t105841 + t105843 + t109282 + t109288;
    t109289
}
