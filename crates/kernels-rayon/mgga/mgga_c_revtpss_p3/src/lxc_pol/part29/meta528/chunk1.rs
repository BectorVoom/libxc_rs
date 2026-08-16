//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1857/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1857(t26169: f64, t6954: f64, t1923: f64, t26204: f64, t6977: f64, t25117: f64, t7349: f64, t1927: f64, t72: f64, t843: f64, t26205: f64, t45958: f64, t7342: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95243 = t6954 * t26169;
    let t95246 = t1923 * t26204 * t6977;
    let t95248 = t25117 * t7349;
    let t95253 = 1232.0_f64 / 81.0_f64 * t1923 * t843 * t72 * t1927;
    let t95255 = t6954 * t26205;
    let t95259 = t45958 * t7342;
    (t95243, t95246, t95248, t95253, t95255, t95259)
}
