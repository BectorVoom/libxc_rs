//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1259/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1259<F: Float>(t14469: F, t50943: F, t50936: F, t3972: F, t3975: F, t9410: F, t13793: F, t53229: F, t13792: F, t8790: F, t13776: F, t37214: F) -> (F, F, F, F, F, F) {
    let t53508 = t50943 * t14469;
    let t53509 = F::new(7.0) / F::new(72.0) * t53508;
    let t53510 = t50936 * t14469;
    let t53513 = t3972 * t3975 * t9410;
    let t53515 = t53229 * t13793;
    let t53516 = F::new(7.0) / F::new(72.0) * t53515;
    let t53517 = t13792 * t8790;
    let t53520 = t13776 * t3975 * t37214;
    (t53509, t53510, t53513, t53516, t53517, t53520)
}
