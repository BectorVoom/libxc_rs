//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 13/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk13(t40: f64, t37: f64, t11: f64, t14: f64, t17: f64, t25: f64, t2: f64, t3: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t43 = 1.0_f64 / (2.0_f64 * t40 - 2.0_f64);
    let t44 = (2.0_f64 * t37 - 2.0_f64) * t43;
    let t46 = 1.0_f64 + 0.278125e-1_f64 * t11;
    let t51 = 0.51785e1_f64 * t14 + 0.905775e0_f64 * t11 + 0.1100325e0_f64 * t17 + 0.1241775e0_f64 * t25;
    let t54 = 1.0_f64 + 0.29608574643216675549e2_f64 / t51;
    let t55 = f64::ln(t54);
    let t56 = t46 * t55;
    let t58 = 0.19751789702565206229e-1_f64 * t44 * t56;
    let t59 = t3 * t2;
    let t60 = 1.0_f64 / t59;
    (t43, t44, t46, t51, t54, t55, t56, t58, t59, t60)
}
