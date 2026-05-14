//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 737/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk737<F: Float>(t8619: F, t8625: F, t7331: F, t7350: F, t7366: F, t8133: F, t8144: F, t8145: F, t8146: F, t8598: F, t8603: F, t8611: F, t8615: F, t8623: F, t9222: F, t8650: F) -> (F, F) {
    let t9226 = 0.28015625e-1 * t8619;
    let t9228 = 7.0 / 144.0 * t8625;
    let t9229 = -t8133 + t7331 + 0.18868855373762491241e-2 * t8598 - 0.37737710747524982483e-2 * t8603 + t9222 + 0.21437009059034868486e-2 * t8611 + 0.12862205435420921092e-2 * t8615 + t7350 - 0.31448092289604152069e-3 * t7366 + t8144 - t8145 + t8146 + t9226 - t8623 / 192.0 + t9228;
    let t9239 = 0.10718504529517434243e-2 * t8650;
    (t9229, t9239)
}
