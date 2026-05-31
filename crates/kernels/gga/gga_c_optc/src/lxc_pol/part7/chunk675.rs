//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 675/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk675<F: Float>(t4328: F, t438: F, t2329: F, t38: F, t620: F, t22: F, t2193: F, t714: F, t34: F, t39: F, t88: F, t35: F, t543: F, rho0: F) -> (F, F, F, F, F, F, F, F) {
    let t4465 = t4328 * t438;
    let t5714 = t2329 * rho0;
    let t6163 = t38 * t620;
    let t6165 = F::cast_from(1.0_f64) / t22 / t6163;
    let t6312 = t2193 * t714;
    let t6316 = t34 * t39;
    let t6318 = F::cast_from(24.0_f64) * t6316 * t88;
    let t6319 = t35 * t543;
    (t4465, t5714, t6163, t6165, t6312, t6316, t6318, t6319)
}
