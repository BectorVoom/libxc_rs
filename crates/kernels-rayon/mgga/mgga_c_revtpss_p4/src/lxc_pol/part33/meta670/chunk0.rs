//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2197/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2197(t29547: f64, t644: f64, t77: f64, t1927: f64, t5872: f64, t2247: f64, t5826: f64, t27154: f64, t98450: f64, t28177: f64, t7898: f64, t28043: f64, t4248: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t108983 = t77 * t29547 * t644;
    let t108986 = t1927 * t5872;
    let t108990 = t2247 * t5826;
    let t109012 = 6.0_f64 * t98450 * t27154;
    let t109014 = 6.0_f64 * t7898 * t28177;
    let t109024 = 4.0_f64 * t4248 * t28043;
    (t108983, t108986, t108990, t109012, t109014, t109024)
}
