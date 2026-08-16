//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 964/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk964(t3379: f64, t4979: f64, t2450: f64, t3371: f64, t4737: f64, t14056: f64, t4419: f64, t3706: f64, t513: f64, t1165: f64, t3290: f64, t3391: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15479 = t3379 * t4979;
    let t15482 = t2450 * t3371;
    let t15483 = t15482 * t4737;
    let t15486 = t14056 * t4419;
    let t15494 = t3706 * t513;
    let t15497 = t3391 * t1165 * t15494 * t3290;
    (t15479, t15482, t15483, t15486, t15494, t15497)
}
