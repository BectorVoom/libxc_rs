//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1270/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1270(t2993: f64, t3001: f64, t33158: f64, t1666: f64, t20461: f64, t27867: f64, t519: f64, t6: f64, t11357: f64, t26017: f64, t11423: f64, t3081: f64, t561: f64) -> (f64, f64, f64, f64) {
    let t35062 = t2993 * t33158 * t3001;
    let t35069 = t2993 * t519 * t20461 * t1666 * t6 * t27867;
    let t35071 = t11357 * t26017;
    let t35074 = t561 * t11423 * t3081;
    (t35062, t35069, t35071, t35074)
}
