//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 938/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk938(t22704: f64, t31091: f64, t81326: f64, t2006: f64, t213: f64, t225: f64, t22633: f64, t22637: f64, t31138: f64, t6883: f64, t31120: f64, t31108: f64, t6897: f64, t794: f64) -> (f64, f64, f64, f64, f64) {
    let t114278 = t22704 * t81326 * t31091;
    let t114279 = 0.3289868133696452873e-1_f64 * t114278;
    let t114285 = t213 * t2006 * t225;
    let t114288 = 0.6579736267392905746e-1_f64 * t22633 * t114285 * t22637;
    let t114291 = t6883 * t31138;
    let t114292 = 0.76763589786250567036e-1_f64 * t114291;
    let t114296 = t6883 * t31120;
    let t114297 = 0.76763589786250567036e-1_f64 * t114296;
    let t114299 = t6897 * t794 * t31108;
    (t114279, t114288, t114292, t114297, t114299)
}
