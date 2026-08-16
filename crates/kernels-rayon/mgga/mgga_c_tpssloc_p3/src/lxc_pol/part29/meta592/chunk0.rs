//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2018/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2018(t22637: f64, t81228: f64, t81326: f64, t22638: f64, t81159: f64, t22892: f64, t6891: f64, t80645: f64, t6892: f64, t81186: f64, t22674: f64, t22934: f64, t6897: f64) -> (f64, f64, f64, f64, f64) {
    let t81328 = t81228 * t81326 * t22637;
    let t81350 = t81159 * t22638;
    let t81365 = t22892 * t80645 * t6891;
    let t81375 = t81186 * t6892;
    let t81379 = t6897 * t22674 * t22934;
    (t81328, t81350, t81365, t81375, t81379)
}
