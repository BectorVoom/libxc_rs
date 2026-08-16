//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2566/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2566(t9575: f64, t9860: f64, t3869: f64, t39538: f64, t39427: f64, t39535: f64, t2496: f64, t9551: f64, t4038: f64, t9372: f64, t1317: f64, t9428: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t47135 = t9860 * t9575;
    let t47138 = 0.43374325201206959368e-1_f64 * t3869 * t39538;
    let t47140 = 0.12842595503380418954e1_f64 * t3869 * t39427;
    let t47142 = 0.38025319932552508021e2_f64 * t3869 * t39535;
    let t47145 = t9551 * t2496;
    let t47147 = t4038 * t9372;
    let t47149 = t1317 * t9428;
    (t47135, t47138, t47140, t47142, t47145, t47147, t47149)
}
