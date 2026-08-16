//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1701/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1701(t1923: f64, t26169: f64, t2047: f64, t25146: f64, t10309: f64, t7342: f64, t38: f64, t624: f64, t2247: f64) -> (f64, f64, f64, f64, f64) {
    let t26170 = t1923 * t26169;
    let t26172 = t2047 * t25146;
    let t26175 = t10309 * t7342;
    let t26178 = t38 * t624;
    let t26179 = t2247 * t26178;
    (t26170, t26172, t26175, t26178, t26179)
}
