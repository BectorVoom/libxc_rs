//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 786/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk786(t173: f64, t9184: f64, t3170: f64, t1027: f64, t1996: f64, t3100: f64, t684: f64, t1917: f64, t1936: f64, t628: f64, t649: f64, t3056: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9185 = t9184 * t173;
    let t9186 = t3170 * t9185;
    let t9188 = t1027 * t1996;
    let t9190 = t3100 * t684;
    let t9192 = t1027 * t1917;
    let t9194 = t628 * t1936;
    let t9195 = t9194 * t649;
    let t9197 = t628 * t3056;
    (t9186, t9188, t9190, t9192, t9195, t9197)
}
