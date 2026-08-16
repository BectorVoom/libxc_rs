//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1145/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1145(t1086: f64, t11990: f64, t15835: f64, t11741: f64, t16133: f64, t3284: f64, t1734: f64, t24759: f64, t1084: f64, t29654: f64, t15680: f64, t26312: f64, t3402: f64) -> (f64, f64, f64, f64, f64) {
    let t34181 = t11990 * t1086 * t15835;
    let t34184 = t11741 * t3284 * t16133;
    let t34186 = t1734 * t24759;
    let t34188 = t1084 * t34186 * t29654;
    let t34191 = t3402 * t26312 * t15680;
    (t34181, t34184, t34186, t34188, t34191)
}
