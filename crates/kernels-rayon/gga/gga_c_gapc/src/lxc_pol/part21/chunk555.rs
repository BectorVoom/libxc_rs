//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 555/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk555(t126: f64, t820: f64, t284: f64, t1063: f64, t828: f64, t3188: f64, t876: f64, t277: f64, t2902: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3201 = t126 * t820;
    let t3202 = t284 * t3201;
    let t3204 = t828 * t1063;
    let t3206 = t3188 * t876;
    let t3207 = t284 * t3206;
    let t3209 = t2902 * t277;
    (t3201, t3202, t3204, t3206, t3207, t3209)
}
