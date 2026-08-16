//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 707/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk707(t2885: f64, t507: f64, t2884: f64, t1412: f64, t472: f64, t144: f64, t653: f64, t1419: f64, t152: f64, t200: f64, t4296: f64, t1603: f64) -> (f64, f64, f64, f64, f64) {
    let t8390 = t2885 * t507;
    let t8391 = t2884 * t8390;
    let t8393 = t1412 * t472;
    let t8394 = t653 * t144;
    let t8396 = t8394 * t152 * t1419;
    let t8397 = t8393 * t8396;
    let t8399 = t4296 * t200;
    let t8400 = t8399 * t1603;
    (t8391, t8394, t8397, t8399, t8400)
}
