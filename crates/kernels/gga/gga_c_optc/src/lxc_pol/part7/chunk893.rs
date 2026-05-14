//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 893/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk893<F: Float>(t3237: F, t9142: F, t3244: F, t2367: F, t3093: F, t1162: F, t8538: F, t914: F, t1179: F, t8505: F, t8521: F, t3126: F, t9073: F, t8469: F, t3103: F, t4435: F, t8513: F, t8518: F, t9116: F, t9119: F, t9122: F, t9125: F, t9128: F, t9130: F, t9134: F, t9136: F, t9139: F) -> (F,) {
    let t9143 = t9142 * t3237;
    let t9144 = t3244 * t9143;
    let t9148 = t2367 * t3093;
    let t9149 = t1162 * t9148;
    let t9151 = t914 * t8538;
    let t9156 = t1179 * t8505;
    let t9158 = t1179 * t8521;
    let t9160 = t9073 * t3126;
    let t9161 = t8469 * t9160;
    let t9164 = 0.34014423178468276541e6 * t9116 * t9119 - 0.34014423178468276541e6 * t9122 * t9125 + 0.99866506516985762611e3 * t9128 * t9130 - 0.19318136643975017455e-1 * t9134 - 0.23229342182245570105e2 * t3103 * t9136 - 0.22720202553012188272e1 * t3244 * t9139 + 0.75734008510040627575e0 * t9144 + 0.30228422675018518374e-1 * t1179 * t8513 - 0.57954409931925052365e-1 * t9149 + 0.17386322979577515709e0 * t1162 * t9151 - 0.50380704458364197289e-1 * t1179 * t8518 - 0.33587136305576131526e-2 * t9156 - 0.20152281783345678915e-1 * t9158 + 0.69688026546736710315e2 * t4435 * t9161;
    (t9164,)
}
