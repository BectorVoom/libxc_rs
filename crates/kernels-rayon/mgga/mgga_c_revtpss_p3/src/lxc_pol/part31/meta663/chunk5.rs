//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2250/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2250(t109172: f64, t109176: f64, t109178: f64, t109180: f64, t109182: f64, t109194: f64, t109196: f64, t109198: f64, t109202: f64, t109231: f64, t109258: f64, t109262: f64, t109266: f64, t109268: f64, t109271: f64, t109274: f64, t2322: f64, t30119: f64, t4254: f64, t4292: f64, t569: f64, t5920: f64, t651: f64, t7221: f64, t7883: f64) -> f64 {
    let t109275 = -t109172 + t109176 - t109178 + t109180 + t109182 - 4.0_f64 * t651 * t7883 * t4292 - 2.0_f64 * t2322 * t30119 - 2.0_f64 * t4254 * t30119 - 2.0_f64 * t651 * t7221 * t5920 - t109194 - t109196 - t109198 + t109202 + (t109231 + t109258) * t569 + t109262 + t109266 - t109268 + t109271 - t109274;
    t109275
}
