//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 705/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk705(t3071: f64, t5: f64, t101: f64, t1459: f64, t3948: f64, t4855: f64, t2902: f64, t3946: f64, t3949: f64, t475: f64, t3938: f64, t1575: f64, t674: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8350 = t5 * t3071;
    let t8351 = t8350 * t101;
    let t8352 = t8351 * t1459;
    let t8353 = t3948 * t4855;
    let t8354 = t8352 * t8353;
    let t8356 = t2902 * t101;
    let t8357 = t8356 * t3946;
    let t8358 = t475 * t3949;
    let t8359 = t8357 * t8358;
    let t8361 = t8351 * t3938;
    let t8362 = t1575 * t674;
    (t8350, t8351, t8352, t8354, t8356, t8359, t8361, t8362)
}
