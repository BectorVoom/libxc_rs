//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 958/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk958(t8469: f64, t9160: f64, t1162: f64, t1179: f64, t3103: f64, t3244: f64, t4435: f64, t8513: f64, t8518: f64, t9116: f64, t9119: f64, t9122: f64, t9125: f64, t9128: f64, t9130: f64, t9134: f64, t9136: f64, t9139: f64, t9144: f64, t9149: f64, t9151: f64, t9156: f64, t9158: f64) -> f64 {
    let t9161 = t8469 * t9160;
    let t9164 = 0.34014423178468276541e6_f64 * t9116 * t9119 - 0.34014423178468276541e6_f64 * t9122 * t9125 + 0.99866506516985762611e3_f64 * t9128 * t9130 - 0.19318136643975017455e-1_f64 * t9134 - 0.23229342182245570105e2_f64 * t3103 * t9136 - 0.22720202553012188272e1_f64 * t3244 * t9139 + 0.75734008510040627575e0_f64 * t9144 + 0.30228422675018518374e-1_f64 * t1179 * t8513 - 0.57954409931925052365e-1_f64 * t9149 + 0.17386322979577515709e0_f64 * t1162 * t9151 - 0.50380704458364197289e-1_f64 * t1179 * t8518 - 0.33587136305576131526e-2_f64 * t9156 - 0.20152281783345678915e-1_f64 * t9158 + 0.69688026546736710315e2_f64 * t4435 * t9161;
    t9164
}
