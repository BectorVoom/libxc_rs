//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2815/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2815(t2439: f64, t2912: f64, t2915: f64, t2909: f64, t11821: f64, t240: f64, t2851: f64, t25273: f64, t268: f64, t271: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41281 = t2439 * t2912;
    let t41285 = t2439 * t2915;
    let t41287 = t2439 * t2909;
    let t41294 = t240 * t11821;
    let t41295 = t2851 * t2851;
    let t41296 = 1.0_f64 / t41295;
    let t41306 = t268 * t25273 * t271;
    (t41281, t41285, t41287, t41294, t41296, t41306)
}
