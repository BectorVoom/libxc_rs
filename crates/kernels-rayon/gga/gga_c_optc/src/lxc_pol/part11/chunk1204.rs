//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1204/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1204(t1162: f64, t18024: f64, t2367: f64, t12860: f64, t15736: f64, t15980: f64, t4501: f64, t15321: f64, t4444: f64, t1179: f64, t54304: f64, t17987: f64, t3244: f64, t9142: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t55623 = t1162 * t2367 * t18024;
    let t55625 = t12860 * t15736;
    let t55637 = t4501 * t15980;
    let t55643 = t4444 * t15321;
    let t55645 = t1179 * t54304;
    let t55734 = t3244 * t9142 * t17987;
    (t55623, t55625, t55637, t55643, t55645, t55734)
}
