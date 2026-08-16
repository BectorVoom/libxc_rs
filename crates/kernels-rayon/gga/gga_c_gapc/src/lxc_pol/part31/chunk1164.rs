//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1164/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1164(t15507: f64, t8: f64, t29867: f64, t332: f64, t6: f64, t7875: f64, t1084: f64, t291: f64, t4052: f64, t3095: f64, t6182: f64, t9438: f64) -> (f64, f64, f64, f64) {
    let t33521 = 1.0_f64 / t8 / t15507;
    let t33527 = t7875 * t332 * t6 * t29867;
    let t33528 = t1084 * t4052 * t33521 * t291 * t33527;
    let t33530 = t3095 * t291;
    let t33532 = t9438 * t33530 * t6182;
    (t33521, t33528, t33530, t33532)
}
