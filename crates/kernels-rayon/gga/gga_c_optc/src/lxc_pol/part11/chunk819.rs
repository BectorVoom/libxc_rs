//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 819/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk819(t4333: f64, t4363: f64, t1128: f64, t5313: f64, t1121: f64, t2586: f64, t5297: f64, t1133: f64, t140: f64, t5255: f64, t871: f64, t464: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15330 = t4363 * t4333;
    let t15332 = t1128 * t5313;
    let t15333 = t1121 * t15332;
    let t15335 = t2586 * t5297;
    let t15336 = t1133 * t15335;
    let t15354 = t5255 * t871 * t140;
    let t15355 = t464 * t15354;
    (t15330, t15332, t15333, t15335, t15336, t15354, t15355)
}
