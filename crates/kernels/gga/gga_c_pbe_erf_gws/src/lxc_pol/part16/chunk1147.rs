//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1147/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1147<F: Float>(t20154: F, t2376: F, t4207: F, t814: F, t14327: F, t3083: F, t53353: F, t1185: F, t14181: F, t14187: F, t14192: F, t27105: F, t4385: F, t51096: F, t52249: F, t52251: F, t52299: F, t53346: F, t53351: F, t53355: F, t53357: F, t53362: F, t8629: F, t8654: F, t8776: F) -> (F,) {
    let t55110 = t20154 * t2376 * t4207 * t814;
    let t55114 = 7.0 / 144.0 * t3083 * t14327;
    let t55117 = 7.0 / 144.0 * t53353;
    let t55124 = -t8629 * t52299 / 24.0 + t8654 * t1185 * t14187 / 24.0 + t8654 * t27105 * t14181 / 24.0 + t8776 * t1185 * t14192 / 32.0 - t4385 * t55110 / 48.0 + t55114 - t53346 / 768.0 - t53351 / 768.0 + t55117 + t53355 / 12.0 + t53357 / 48.0 + t53362 / 384.0 - 7.0 / 1152.0 * t51096 - 7.0 / 144.0 * t52249 + 35.0 / 108.0 * t52251;
    (t55124,)
}
