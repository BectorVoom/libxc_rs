//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 838/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk838(t100: f64, t1504: f64, t2182: f64, t6868: f64, t810: f64, t3205: f64, t858: f64, t893: f64, t3065: f64, t2416: f64, t891: f64, t2081: f64, t326: f64, t6469: f64, param_gamma: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8335 = t1504 * t100;
    let t8524 = param_gamma * t2182;
    let t8556 = t6868 * t810;
    let t8599 = t3205 * t858;
    let t8605 = t858 * t893;
    let t8606 = t3065 * t8605;
    let t8734 = t891 * t2416;
    let t8782 = t326 * t6469 * t2081;
    (t8335, t8524, t8556, t8599, t8606, t8734, t8782)
}
