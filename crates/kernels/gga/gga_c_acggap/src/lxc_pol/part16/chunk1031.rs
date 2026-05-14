//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1031/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1031<F: Float>(t31471: F, t31473: F, t31477: F, t31479: F, t35624: F, t35632: F, t35636: F, t35647: F, t37645: F, t37652: F, t37653: F, t40083: F, t40086: F, t40089: F, t40092: F, t40095: F, t40099: F, t40101: F) -> (F,) {
    let t40103 = t35624 - t35632 + t35636 - 7.0 / 72.0 * t40083 - 0.22921875e-1 * t40086 - 0.4584375e-1 * t40089 - 0.21437009059034868486e-2 * t40092 + 0.10718504529517434243e-2 * t40095 - t31471 + t31473 + t37645 - t35647 - 0.65369791666666666667e-1 * t31477 + 0.66040993808168719343e-2 * t31479 + 0.10718504529517434243e-3 * t40099 - 0.51448821741683684367e-1 * t40101 - t37652 - t37653;
    (t40103,)
}
