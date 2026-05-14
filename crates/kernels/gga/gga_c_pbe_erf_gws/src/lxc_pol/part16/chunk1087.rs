//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1087/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1087<F: Float>(t2112: F, t2306: F, t3972: F, t3975: F, t9385: F, t13780: F, t13859: F, t3990: F, t8764: F, t14733: F, t4390: F, t3038: F, t9520: F, t1176: F, t14639: F, t6365: F, t923: F) -> (F, F, F, F, F) {
    let t53362 = t3972 * t3975 * t9385 * t2306 * t2112;
    let t53378 = t13859 * t3990 * t13780 * t8764;
    let t53386 = t14733 * t4390;
    let t53395 = t3972 * t3975 * t3038 * t9520;
    let t53424 = t1176 * t923 * t6365 * t14639;
    (t53362, t53378, t53386, t53395, t53424)
}
