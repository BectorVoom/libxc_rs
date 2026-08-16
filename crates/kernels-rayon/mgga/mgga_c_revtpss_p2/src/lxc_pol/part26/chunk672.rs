//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 672/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk672(t30: f64, t890: f64, t33: f64, t775: f64, t1315: f64, t196: f64, t197: f64) -> (f64, f64, f64, f64, f64) {
    let t7092 = t30 * t890;
    let t7200 = t33 * t775;
    let t7207 = t33 * t890;
    let t7234 = t1315 * t196;
    let t7235 = t7234 * t197;
    (t7092, t7200, t7207, t7234, t7235)
}
