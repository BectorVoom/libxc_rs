//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1150/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1150(t311: f64, t34159: f64, t7089: f64, t919: f64, t2415: f64, t3439: f64, t9756: f64, t1086: f64, t11790: f64, t23104: f64, t11449: f64, t11805: f64, t190: f64, t761: f64) -> (f64, f64, f64, f64) {
    let t34235 = t311 * t7089 * t34159 * t919;
    let t34238 = t9756 * t2415 * t3439;
    let t34241 = t11790 * t1086 * t23104;
    let t34245 = t761 * t190 * t11449 * t11805;
    (t34235, t34238, t34241, t34245)
}
