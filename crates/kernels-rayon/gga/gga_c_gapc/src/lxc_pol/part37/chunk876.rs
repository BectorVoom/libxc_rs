//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 876/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk876(t277: f64, t5294: f64, t291: f64, t3694: f64, t3439: f64, t3273: f64, t9529: f64, t1092: f64, t2486: f64, t7182: f64, t906: f64, t904: f64) -> (f64, f64, f64, f64, f64) {
    let t9957 = t277 * t5294;
    let t9958 = t3694 * t291;
    let t9959 = t9958 * t3439;
    let t9960 = t9957 * t9959;
    let t9962 = t9529 * t3273;
    let t9964 = t1092 * t2486;
    let t9966 = t7182 * t906;
    let t9967 = t904 * t9966;
    (t9959, t9960, t9962, t9964, t9967)
}
