//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1154/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1154(t18736: f64, t20540: f64, t2365: f64, t20692: f64, t7025: f64, t4130: f64, t874: f64, t6907: f64, t9272: f64, t1265: f64, t587: f64, t9438: f64, t9439: f64) -> (f64, f64, f64, f64) {
    let t31175 = 0.59584149919750711116e-1_f64 * t18736 * t2365 * t20540;
    let t31178 = 0.59584149919750711116e-1_f64 * t7025 * t2365 * t20692;
    let t31187 = t4130 * t874;
    let t31190 = 0.10352590007558602413e2_f64 * t9272 * t31187 * t6907;
    let t31207 = t587 * t9438 * t9439 * t1265;
    (t31175, t31178, t31190, t31207)
}
