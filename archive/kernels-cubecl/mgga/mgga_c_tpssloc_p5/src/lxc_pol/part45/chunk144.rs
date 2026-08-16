//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 144/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk144<F: Float>(t475: F, t46: F, t47: F, rho1: F, sigma2: F) -> (F, F, F, F, F) {
    let t476 = t475 - F::cast_from(1.0_f64);
    let t477 = F::cast_from(1.0_f64) / t476;
    let t478 = sigma2 * sigma2;
    let t479 = t477 * t478;
    let t480 = t46 * t46;
    let t481 = t480 * rho1;
    let t483 = F::cast_from(1.0_f64) / t47 / t481;
    (t476, t477, t478, t479, t483)
}
