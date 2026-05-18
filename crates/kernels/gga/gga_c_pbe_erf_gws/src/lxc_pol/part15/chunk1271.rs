//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1271/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1271<F: Float>(t4182: F, t6781: F, t829: F, t830: F, t13791: F, t3039: F, t13984: F, t14657: F, t51714: F, t13793: F, t51584: F, t13939: F, t3040: F, t51063: F, t51561: F, t51564: F, t53664: F, t53666: F, t53668: F, t53671: F, t53675: F, t53677: F, t827: F, t8793: F) -> F {
    let t53679 = t6781 * t4182;
    let t53681 = t829 * t830 * t53679;
    let t53688 = t3039 * t13791;
    let t53689 = t53688 * t13984;
    let t53691 = t14657 * t51714;
    let t53693 = t53688 * t13793;
    let t53695 = t14657 * t51584;
    let t53697 = -t53664 / F::new(384.0) - t53666 - t53668 / F::new(768.0) - t53671 / F::new(1536.0) - t3040 * t13939 / F::new(48.0) + t53675 / F::new(8.0) - t53677 / F::new(48.0) - t827 * t53681 / F::new(48.0) - F::new(7.0) / F::new(288.0) * t51561 + F::new(7.0) / F::new(1152.0) * t51564 + t8793 * t51063 / F::new(48.0) - t53689 / F::new(48.0) - t53691 / F::new(96.0) - t53693 / F::new(24.0) + t53695 / F::new(48.0);
    t53697
}
