//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1165/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1165<F: Float>(t11094: F, t1637: F, t14257: F, t14262: F, t14376: F, t14378: F, t14381: F, t14384: F, t14387: F, t14391: F, t14394: F, t14398: F, t14424: F, t14472: F, t14475: F, t14477: F, t14479: F, t14482: F, t14484: F, t14486: F, t3209: F, t3213: F, t4700: F, t4701: F) -> F {
    let t14667 = t1637 * t11094;
    let t14673 = F::cast_from(2.0_f64) * t14667 * t3213 * t4700 - t3209 * t4700 * t4701 - t14257 - t14262 - t14376 + t14378 - t14381 - t14384 - t14387 + t14391 + t14394 + t14398 + t14424 + t14472 - t14475 - t14477 + t14479 - t14482 - t14484 - t14486;
    t14673
}
