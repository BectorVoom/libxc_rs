//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2271/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2271(t105818: f64, t105822: f64, t105826: f64, t105830: f64, t105834: f64, t105837: f64, t105839: f64, t105841: f64, t105843: f64, t109282: f64, t109288: f64, t2170: f64, t22556: f64, t22559: f64, t22565: f64, t22568: f64, t5802: f64, t6945: f64, t7696: f64, t8245: f64) -> f64 {
    let t113039 = 6.0_f64 * t2170 * t22556 + 12.0_f64 * t2170 * t22559 + 6.0_f64 * t2170 * t22565 + 3.0_f64 * t2170 * t22568 + 12.0_f64 * t5802 * t8245 + 6.0_f64 * t6945 * t7696 + t105818 + t105822 + t105826 + t105830 + t105834 + t105837 + t105839 + t105841 + t105843 + t109282 + t109288;
    t113039
}
