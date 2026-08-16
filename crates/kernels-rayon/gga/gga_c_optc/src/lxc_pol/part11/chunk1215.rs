//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1215/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1215(t47886: f64, t4741: f64, t4756: f64, t47938: f64, t28489: f64, t1303: f64, t1317: f64, t13504: f64, t16221: f64, t16292: f64, t16579: f64, t201: f64, t22074: f64, t3316: f64, t3318: f64, t37325: f64, t37328: f64, t37341: f64, t4611: f64, t4759: f64, t714: f64, t95: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t55944 = 0.73246220147012639764e-3_f64 * t47886;
    let t55945 = t4741 * t4741;
    let t55951 = t4756 * t4756;
    let t55977 = 0.23392893589820816284e1_f64 * t47938;
    let t55980 = 0.22787712934626154593e-2_f64 * t28489;
    let t55981 = 2.0_f64 * t3316 * t3318 * t16292 * t1317 * t201 + 3.0_f64 * t3316 * t3318 * t4756 * t4741 * t201 + 0.62027715443768233192e-1_f64 * t95 * t16221 * t1303 * t714 + 3.0_f64 * t4611 * t4759 + 6.0_f64 * t13504 * t16579 + 70.0_f64 / 3.0_f64 * t37325 - t55977 + 6.0_f64 * t37328 - 14.0_f64 * t37341 + t22074 - t55980;
    (t55944, t55945, t55951, t55977, t55980, t55981)
}
