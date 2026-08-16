//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1231/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1231<F: Float>(t1902: F, t828: F, t3701: F, t6995: F, t1351: F, t2006: F, t22811: F, t604: F, t9226: F, t2233: F, t2239: F, t601: F, t9238: F) -> (F, F, F, F, F, F, F) {
    let t30684 = t1902 * t828;
    let t31035 = t3701 * t6995;
    let t31201 = t2006 * t1351;
    let t39041 = F::cast_from(1.0_f64) / t22811;
    let t39046 = t9226 * t604;
    let t39049 = t2233 * t2239;
    let t39054 = t601 * t9238;
    (t30684, t31035, t31201, t39041, t39046, t39049, t39054)
}
