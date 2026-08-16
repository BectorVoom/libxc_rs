//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 792/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk792(t2568: f64, t9454: f64, t291: f64, t7549: f64, t8820: f64, t7547: f64, t871: f64, t903: f64, t2526: f64, t2505: f64, t904: f64, t1: f64, t282: f64, t3: f64) -> (f64, f64, f64, f64, f64) {
    let t9457 = t2568 * t9454;
    let t9460 = t8820 * t291 * t7549;
    let t9461 = t7547 * t9460;
    let t9463 = t871 * t903;
    let t9464 = t9463 * t2526;
    let t9468 = t904 * t2505;
    let t9471 = t282 * t1 * t3;
    (t9457, t9461, t9464, t9468, t9471)
}
