//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2227/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2227(t109041: f64, t109043: f64, t109045: f64, t109047: f64, t109049: f64, t109052: f64, t109054: f64, t109058: f64, t109060: f64, t109063: f64, t109074: f64, t109078: f64, t109081: f64, t1518: f64, t18242: f64, t2322: f64, t27060: f64, t29337: f64, t29432: f64, t30963: f64, t4254: f64, t5921: f64, t651: f64, t7586: f64) -> f64 {
    let t111762 = -4.0_f64 * t1518 * t29337 * t651 - 2.0_f64 * t18242 * t7586 - 4.0_f64 * t2322 * t30963 - 2.0_f64 * t27060 * t5921 - 2.0_f64 * t29432 * t5921 - 4.0_f64 * t30963 * t4254 - t109041 - t109043 - t109045 - t109047 + t109049 + t109052 - t109054 - t109058 - t109060 - t109063 + t109074 + t109078 - t109081;
    t111762
}
