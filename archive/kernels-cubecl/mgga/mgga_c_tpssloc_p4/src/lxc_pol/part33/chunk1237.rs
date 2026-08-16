//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1237/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1237<F: Float>(t10336: F, t1920: F, t1922: F, t10164: F, t225: F, t3034: F, t336: F, t131: F, t350: F, t38: F, t10469: F, t344: F) -> (F, F, F, F) {
    let t82436 = F::cast_from(0.30461741978670859935e-2_f64) * t1920 * t10336 * t1922;
    let t82481 = t225 * t10164;
    let t82510 = F::cast_from(1.0_f64) / t3034 / t336;
    let t82513 = t38 * t82510 * t131 * t350;
    let t82514 = t344 * t10469;
    (t82436, t82481, t82513, t82514)
}
