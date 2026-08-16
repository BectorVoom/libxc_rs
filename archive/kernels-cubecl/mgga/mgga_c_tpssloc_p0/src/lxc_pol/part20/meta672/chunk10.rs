//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2536/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2536<F: Float>(t51039: F, t51051: F, t51034: F, t51037: F, t51041: F, t51043: F, t51046: F, t51049: F, t51053: F, t51056: F, t51100: F, t51102: F) -> F {
    let t51349 = F::cast_from(0.69463333333333333334e0_f64) * t51039;
    let t51354 = F::cast_from(0.11577222222222222222e0_f64) * t51051;
    let t51359 = F::cast_from(0.13892666666666666667e0_f64) * t51034 - F::cast_from(0.62517e0_f64) * t51037 + t51349 - F::cast_from(0.41678000000000000001e0_f64) * t51041 - F::cast_from(0.125034e1_f64) * t51043 - F::cast_from(0.104195e0_f64) * t51046 - F::cast_from(0.62517000000000000001e0_f64) * t51049 - t51354 - F::cast_from(0.83356000000000000001e0_f64) * t51053 + F::cast_from(0.62517e0_f64) * t51056 + F::cast_from(0.3529725e1_f64) * t51100 + F::cast_from(0.6311625e0_f64) * t51102;
    t51359
}
