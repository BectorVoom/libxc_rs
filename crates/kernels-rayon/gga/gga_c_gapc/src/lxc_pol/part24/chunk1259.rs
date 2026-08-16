//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1259/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1259(t11626: f64, t3234: f64, t6179: f64, t11625: f64, t11669: f64, t2440: f64, t3728: f64, t7029: f64, t11675: f64, t24271: f64, t10349: f64, t11694: f64, t332: f64) -> (f64, f64, f64, f64, f64) {
    let t35823 = t3234 * t6179 * t11626;
    let t35826 = t11625 * t11669 * t2440;
    let t35829 = t11625 * t3728 * t7029;
    let t35831 = t11675 * t24271;
    let t35834 = t11694 * t332 * t10349;
    (t35823, t35826, t35829, t35831, t35834)
}
