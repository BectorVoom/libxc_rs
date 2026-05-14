//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 998/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk998<F: Float>(t1839: F, t372: F, t1181: F, t2068: F, t604: F, t6283: F, t7332: F, t6255: F, t7561: F, t6260: F, t30543: F, t9720: F, t30793: F, t30798: F, t30830: F, t30854: F, t30862: F, t34991: F, t37345: F, t39581: F, t39585: F, t39587: F, t39592: F, t39594: F) -> (F, F) {
    let t39596 = t1839 * t372;
    let t39599 = t2068 * t1181 * t604 * t39596;
    let t39601 = t7332 * t6283;
    let t39605 = t7561 * t6255;
    let t39607 = t7561 * t6260;
    let t39609 = t30543 * t9720;
    let t39614 = 0.10718504529517434243e-2 * t39581 + 0.7145669686344956162e-3 * t39585 - 0.34299214494455789578e-2 * t39587 - 0.62896184579208304136e-3 * t39592 + t34991 + 0.34299214494455789578e-2 * t39594 + 0.21437009059034868486e-3 * t39599 + t39601 / 16.0 - 0.18868855373762491241e-2 * t30793 - 0.10718504529517434243e-3 * t30798 + 0.34299214494455789578e-2 * t39605 + 0.34299214494455789578e-2 * t39607 - 0.18868855373762491241e-1 * t39609 - 0.10482697429868050689e-2 * t30830 + 0.12862205435420921092e-2 * t30854 + t37345 - 0.6431102717710460546e-2 * t30862;
    (t39596, t39614)
}
