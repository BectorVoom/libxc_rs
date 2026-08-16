//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2212/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2212(t109269: f64, t28199: f64, t25082: f64, t27153: f64, t33651: f64, t6941: f64, t7331: f64, t5795: f64, t7950: f64, t7953: f64, t1916: f64, t28265: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t109271 = 4.0_f64 * t109269 * t28199;
    let t109274 = 6.0_f64 * t25082 * t33651 * t27153;
    let t109282 = 6.0_f64 * t6941 * t7331;
    let t109288 = 12.0_f64 * t5795 * t7950;
    let t109291 = 6.0_f64 * t5795 * t7953;
    let t109293 = 12.0_f64 * t1916 * t28265;
    (t109271, t109274, t109282, t109288, t109291, t109293)
}
