//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 504/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk504(t1362: f64, t3920: f64, t1386: f64, t820: f64, t843: f64, t1401: f64, t241: f64, t1412: f64, t72: f64, t245: f64) -> (f64, f64, f64, f64, f64) {
    let t3922 = 0.13009920719177044025e-1_f64 * t1362 * t3920;
    let t3930 = t820 * t1386 * t843;
    let t3931 = t3930 * t1401;
    let t3934 = t820 * t1386 * t241;
    let t3935 = t1412 * t72;
    let t3936 = t3935 * t245;
    (t3922, t3930, t3931, t3934, t3936)
}
