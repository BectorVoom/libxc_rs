//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 388/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk388(t1875: f64, t612: f64, t583: f64, t618: f64, t617: f64, t122: f64, t653: f64, t128: f64) -> (f64, f64, f64, f64, f64) {
    let t1876 = t1875 * t612;
    let t1877 = t618 * t583;
    let t1878 = t617 * t1877;
    let t1881 = t122 * t653;
    let t1882 = t1881 * t128;
    (t1876, t1877, t1878, t1881, t1882)
}
