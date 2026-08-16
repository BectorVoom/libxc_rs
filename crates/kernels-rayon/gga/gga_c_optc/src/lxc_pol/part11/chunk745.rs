//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 745/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk745(t1244: f64, t1871: f64, t40: f64, t3546: f64, t740: f64, t1310: f64, t2204: f64, t4: f64) -> (f64, f64, f64, f64, f64) {
    let t9521 = t1244 * t1871;
    let t9522 = t40 * t9521;
    let t9523 = t3546 * t740;
    let t9527 = t1310 * t2204;
    let t9529 = t1244 * t4;
    (t9521, t9522, t9523, t9527, t9529)
}
