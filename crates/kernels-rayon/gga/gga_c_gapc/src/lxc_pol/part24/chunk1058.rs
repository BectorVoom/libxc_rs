//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1058/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1058(t1908: f64, t505: f64, t647: f64, t8715: f64, t2999: f64, t5216: f64, t1648: f64, t3005: f64, t154: f64, t3949: f64, t126: f64, t632: f64) -> (f64, f64, f64, f64, f64) {
    let t27754 = t505 * t1908 * t647 * t8715;
    let t27867 = t2999 * t5216;
    let t27868 = t1648 * t3005 * t27867;
    let t27889 = t154 * t3949;
    let t27935 = t632 * t126;
    (t27754, t27867, t27868, t27889, t27935)
}
