//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1132/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1132<F: Float>(t1125: F, t51335: F, t14535: F, t2087: F, t3291: F, t51214: F, t14007: F, t9485: F, t53994: F, t53996: F, t53998: F, t54000: F, t54002: F, t54004: F, t54006: F, t54008: F) -> (F,) {
    let t54010 = t1125 * t51335;
    let t54012 = t2087 * t14535;
    let t54014 = t51214 * t3291;
    let t54015 = 7.0 / 576.0 * t54014;
    let t54016 = t14007 * t9485;
    let t54018 = t53994 / 32.0 + t53996 / 24.0 + t53998 / 24.0 - t54000 / 192.0 - t54002 / 384.0 + t54004 / 24.0 - t54006 / 48.0 - t54008 / 96.0 + t54010 / 16.0 - t54012 / 48.0 + t54015 + t54016 / 192.0;
    (t54018,)
}
