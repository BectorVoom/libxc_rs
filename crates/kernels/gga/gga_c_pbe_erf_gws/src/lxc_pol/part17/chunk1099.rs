//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1099/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1099<F: Float>(t3038: F, t3972: F, t3975: F, t9520: F, t14643: F, t840: F, t14793: F, t9270: F, t1144: F, t13909: F, t859: F, t1176: F, t14639: F, t6365: F, t923: F, t3959: F, t8766: F) -> (F, F, F, F, F, F) {
    let t53395 = t3972 * t3975 * t3038 * t9520;
    let t53405 = 7.0 / 144.0 * t840 * t14643;
    let t53407 = 7.0 / 24.0 * t9270 * t14793;
    let t53419 = t859 * t1144 * t13909;
    let t53424 = t1176 * t923 * t6365 * t14639;
    let t53425 = 35.0 / 576.0 * t53424;
    let t53426 = t3959 * t8766;
    (t53395, t53405, t53407, t53419, t53425, t53426)
}
