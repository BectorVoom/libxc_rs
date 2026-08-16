//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 667/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk667(t1615: f64, t572: f64, t574: f64, t177: f64) -> (f64, f64, f64, f64) {
    let t4908 = t572 * t1615;
    let t4913 = t574 * t574;
    let t4914 = 1.0_f64 / t4913;
    let t4915 = t177 * t4914;
    (t4908, t4913, t4914, t4915)
}
