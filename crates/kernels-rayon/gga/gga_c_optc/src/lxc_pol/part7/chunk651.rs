//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 651/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk651(t50: f64, t1900: f64, t104: f64, t95: f64, t176: f64, t185: f64, t102: f64, t108: f64, t110: f64, t115: f64, t56: f64, t5: f64, t1: f64, t2060: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t51 = t50 <= zeta_threshold;
    let t3298 = piecewise3(t51, 0.0_f64, t1900);
    let t3308 = t95 * t104;
    let t3313 = t176 * t185;
    let t3314 = t102 * t108;
    let t3315 = t3314 * t110;
    let t3316 = t3313 * t3315;
    let t3317 = t56 * t115;
    let t3318 = t3317 * t5;
    let t3411 = t2060 * t1;
    (t3298, t3308, t3313, t3314, t3315, t3316, t3318, t3411)
}
