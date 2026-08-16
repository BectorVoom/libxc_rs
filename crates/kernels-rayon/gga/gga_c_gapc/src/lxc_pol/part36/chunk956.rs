//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 956/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk956(t3679: f64, t5248: f64, t1643: f64, t3683: f64, t424: f64, t205: f64, t3680: f64, t5252: f64, t3091: f64, t3670: f64, t19: f64, t515: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11566 = t3679 * t5248;
    let t11567 = t1643 * t11566;
    let t11569 = t424 * t3683;
    let t11570 = t11569 * t205;
    let t11572 = t5252 * t3680;
    let t11574 = t3670 * t3091;
    let t11576 = t515 * t19;
    (t11566, t11567, t11569, t11570, t11572, t11574, t11576)
}
