//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2236/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2236(t5517: f64, t651: f64, t7741: f64, t101417: f64, t2014: f64, t7900: f64, t109035: f64, t109038: f64, t109039: f64, t109041: f64, t109043: f64, t109045: f64, t109047: f64, t109049: f64, t109052: f64, t109054: f64, t109058: f64, t109060: f64, t1518: f64, t2322: f64, t27830: f64, t29986: f64, t30116: f64, t33602: f64, t4254: f64, t4293: f64, t649: f64) -> f64 {
    let t109063 = 4.0_f64 * t651 * t5517 * t7741;
    let t109074 = 6.0_f64 * t2014 * t101417 * t7900;
    let t109075 = -4.0_f64 * t1518 * t27830 * t651 - 4.0_f64 * t2322 * t30116 - t29986 * t649 - 4.0_f64 * t30116 * t4254 - 4.0_f64 * t33602 * t4293 - t109035 - t109038 - t109039 - t109041 - t109043 - t109045 - t109047 + t109049 + t109052 - t109054 - t109058 - t109060 - t109063 + t109074;
    t109075
}
