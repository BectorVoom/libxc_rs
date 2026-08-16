//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2120/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2120<F: Float>(t25097: F, t81782: F, t81783: F, t1516: F, t81769: F, t23133: F, t4261: F, t25111: F, t25115: F, t87229: F, t23132: F, t4166: F) -> (F, F, F, F, F, F) {
    let t87328 = t81782 * t81783 * t25097;
    let t87329 = F::cast_from(0.40372756094140390854e-3_f64) * t87328;
    let t87330 = t81769 * t1516;
    let t87331 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t87330;
    let t87332 = t23133 * t4261;
    let t87333 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t87332;
    let t87335 = t81782 * t81783 * t25111;
    let t87336 = F::cast_from(0.40372756094140390854e-3_f64) * t87335;
    let t87338 = t87229 * t81783 * t25115;
    let t87339 = F::cast_from(0.6728792682356731809e-4_f64) * t87338;
    let t87340 = t4166 * t23132;
    (t87329, t87331, t87333, t87336, t87339, t87340)
}
