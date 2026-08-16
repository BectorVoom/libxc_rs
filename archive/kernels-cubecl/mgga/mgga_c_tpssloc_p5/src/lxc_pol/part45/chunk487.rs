//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 487/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk487<F: Float>(t1230: F, t248: F, t3252: F, t3248: F, t1009: F, t1190: F, t1011: F, t1212: F, t374: F, t486: F, t677: F, t485: F) -> (F, F, F, F, F) {
    let t3527 = t248 * t1230 * t3252;
    let t3531 = t248 * t1230 * t3248;
    let t3534 = t1190 * t1009;
    let t3535 = t3534 * t1011;
    let t3536 = t3535 * t1212;
    let t3540 = t374 * t677 * t486;
    let t3542 = t485 * t3540 / F::cast_from(13824.0_f64);
    (t3527, t3531, t3534, t3536, t3542)
}
