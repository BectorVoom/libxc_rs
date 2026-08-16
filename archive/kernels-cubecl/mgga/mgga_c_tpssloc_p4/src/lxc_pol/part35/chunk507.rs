//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 507/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk507<F: Float>(t3236: F, t1146: F, t445: F, t440: F, t3293: F) -> (F, F, F, F, F, F, F) {
    let t3363 = F::cast_from(0.12361111111111111111e-1_f64) * t3236;
    let t3374 = t1146 * t445;
    let t3375 = F::cast_from(1.0_f64) / t3374;
    let t3376 = t440 * t3375;
    let t3383 = F::cast_from(0.40256666666666666667e0_f64) * t3236;
    let t3390 = F::cast_from(0.137975e0_f64) * t3293;
    let t3399 = t1146 * t1146;
    let t3400 = F::cast_from(1.0_f64) / t3399;
    (t3363, t3375, t3376, t3383, t3390, t3399, t3400)
}
