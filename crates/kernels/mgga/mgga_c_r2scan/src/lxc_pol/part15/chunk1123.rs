//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1123/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1123<F: Float>(t37584: F, t37588: F, t37600: F, t39452: F, t39455: F, t39459: F, t39460: F, t39462: F, t39464: F, t39467: F, t39470: F, t39476: F) -> F {
    let t39478 = F::new(0.17336443480108537126e0) * t39452 - F::new(0.5200933044032561138e0) * t39455 - t39459 + F::new(0.86682217400542685632e-1) * t39460 + F::new(0.2600466522016280569e0) * t39462 - F::new(0.59512461497092438715e-1) * t39464 + F::new(0.5200933044032561138e0) * t39467 - F::new(0.14457274399185490173e-3) * t39470 - F::new(0.28565981518604370583e-1) * t37584 - F::new(0.47609969197673950972e-2) * t37588 - t37600 - F::new(0.21831846657716620896e-2) * t39476;
    t39478
}
