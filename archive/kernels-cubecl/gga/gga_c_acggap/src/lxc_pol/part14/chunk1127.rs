//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1127/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1127<F: Float>(t30543: F, t9720: F, t30793: F, t30798: F, t30830: F, t30854: F, t30862: F, t34991: F, t37345: F, t39581: F, t39585: F, t39587: F, t39592: F, t39594: F, t39599: F, t39601: F, t39605: F, t39607: F) -> F {
    let t39609 = t30543 * t9720;
    let t39614 = F::cast_from(0.10718504529517434243e-2_f64) * t39581 + F::cast_from(0.7145669686344956162e-3_f64) * t39585 - F::cast_from(0.34299214494455789578e-2_f64) * t39587 - F::cast_from(0.62896184579208304136e-3_f64) * t39592 + t34991 + F::cast_from(0.34299214494455789578e-2_f64) * t39594 + F::cast_from(0.21437009059034868486e-3_f64) * t39599 + t39601 / F::cast_from(16.0_f64) - F::cast_from(0.18868855373762491241e-2_f64) * t30793 - F::cast_from(0.10718504529517434243e-3_f64) * t30798 + F::cast_from(0.34299214494455789578e-2_f64) * t39605 + F::cast_from(0.34299214494455789578e-2_f64) * t39607 - F::cast_from(0.18868855373762491241e-1_f64) * t39609 - F::cast_from(0.10482697429868050689e-2_f64) * t30830 + F::cast_from(0.12862205435420921092e-2_f64) * t30854 + t37345 - F::cast_from(0.6431102717710460546e-2_f64) * t30862;
    t39614
}
