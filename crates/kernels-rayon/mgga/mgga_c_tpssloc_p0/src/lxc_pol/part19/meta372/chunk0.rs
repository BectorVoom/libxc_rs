//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1380/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1380(t3637: f64, t3639: f64, t11153: f64, t2244: f64, t2250: f64, t136: f64, t3297: f64, t11158: f64, t9258: f64, t3243: f64, t1113: f64, t11167: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t43703 = t3637 * t3637;
    let t43705 = t3639 * t3639;
    let t43706 = 1.0_f64 / t43705;
    let t43711 = t11153 * t2244 * t2250;
    let t43713 = t136 * t3297 * t43711;
    let t43715 = t11158 * t9258;
    let t43717 = t136 * t3297 * t43715;
    let t43719 = t3243 * t2250;
    let t43721 = t136 * t1113 * t43719;
    let t43723 = t11167 * t9258;
    (t43703, t43706, t43711, t43713, t43715, t43717, t43719, t43721, t43723)
}
