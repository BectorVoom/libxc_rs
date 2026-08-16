//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 916/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk916(t10001: f64, t22182: f64, t6800: f64, t72: f64, t757: f64, t1317: f64, t6801: f64, t1320: f64, t749: f64, t512: f64, t177: f64, t762: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22183 = t10001 * t22182;
    let t22185 = t6800 * t72;
    let t22186 = t22185 * t757;
    let t22188 = t1317 * t6801;
    let t22191 = t1320 * t6801;
    let t22195 = t6800 * t749;
    let t22196 = t512 * t22195;
    let t22212 = t6800 * t177;
    let t22213 = t22212 * t762;
    (t22183, t22186, t22188, t22191, t22196, t22213)
}
