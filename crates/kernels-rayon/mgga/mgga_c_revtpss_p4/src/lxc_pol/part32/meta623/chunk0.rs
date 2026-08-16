//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1966/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1966(t29547: f64, t644: f64, t77: f64, t1927: f64, t5872: f64, t2247: f64, t5826: f64, t196: f64, t197: f64, t22525: f64, t1448: f64, t6781: f64) -> (f64, f64, f64, f64, f64) {
    let t108983 = t77 * t29547 * t644;
    let t108986 = t1927 * t5872;
    let t108990 = t2247 * t5826;
    let t109077 = t22525 * t196 * t197;
    let t109096 = t6781 * t1448;
    (t108983, t108986, t108990, t109077, t109096)
}
