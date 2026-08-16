//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 904/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk904(t1762: f64, t268: f64, t188: f64, t826: f64, t2531: f64, t3239: f64, t1936: f64, t2493: f64, t3243: f64, t6182: f64, t772: f64, t10153: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10243 = t1762 * t268;
    let t10244 = t826 * t188;
    let t10245 = t10243 * t10244;
    let t10246 = t3239 * t2531;
    let t10247 = t10245 * t10246;
    let t10249 = t1936 * t2493;
    let t10250 = t3243 * t10249;
    let t10252 = t772 * t6182;
    let t10253 = t10153 * t10252;
    (t10243, t10245, t10246, t10247, t10250, t10253)
}
