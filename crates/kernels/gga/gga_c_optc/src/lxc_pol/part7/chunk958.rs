//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 958/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk958<F: Float>(t8469: F, t9160: F, t1162: F, t1179: F, t3103: F, t3244: F, t4435: F, t8513: F, t8518: F, t9116: F, t9119: F, t9122: F, t9125: F, t9128: F, t9130: F, t9134: F, t9136: F, t9139: F, t9144: F, t9149: F, t9151: F, t9156: F, t9158: F) -> F {
    let t9161 = t8469 * t9160;
    let t9164 = F::cast_from(0.34014423178468276541e6_f64) * t9116 * t9119 - F::cast_from(0.34014423178468276541e6_f64) * t9122 * t9125 + F::cast_from(0.99866506516985762611e3_f64) * t9128 * t9130 - F::cast_from(0.19318136643975017455e-1_f64) * t9134 - F::cast_from(0.23229342182245570105e2_f64) * t3103 * t9136 - F::cast_from(0.22720202553012188272e1_f64) * t3244 * t9139 + F::cast_from(0.75734008510040627575e0_f64) * t9144 + F::cast_from(0.30228422675018518374e-1_f64) * t1179 * t8513 - F::cast_from(0.57954409931925052365e-1_f64) * t9149 + F::cast_from(0.17386322979577515709e0_f64) * t1162 * t9151 - F::cast_from(0.50380704458364197289e-1_f64) * t1179 * t8518 - F::cast_from(0.33587136305576131526e-2_f64) * t9156 - F::cast_from(0.20152281783345678915e-1_f64) * t9158 + F::cast_from(0.69688026546736710315e2_f64) * t4435 * t9161;
    t9164
}
