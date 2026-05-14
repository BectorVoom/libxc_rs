//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1155/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1155<F: Float>(t1205: F, t20173: F, t53645: F, t14918: F, t2367: F, t1115: F, t14185: F, t14321: F, t2408: F, t3066: F, t35566: F, t51530: F, t52350: F, t52353: F, t53631: F, t53639: F, t53643: F, t53664: F, t53668: F, t53671: F, t53675: F, t9283: F, t9297: F, t9702: F) -> (F,) {
    let t55297 = t20173 * t1205;
    let t55311 = 7.0 / 72.0 * t53645;
    let t55315 = 7.0 / 144.0 * t2367 * t14918;
    let t55321 = -35.0 / 216.0 * t52353 - t53631 / 192.0 + t3066 * t9283 * t55297 * t9297 / 4.0 + t53639 / 1536.0 - t2408 * t35566 * t14321 / 12.0 - t2408 * t9283 * t14185 * t9702 / 12.0 + t53643 / 768.0 - t55311 - t1115 * t52350 / 96.0 + t55315 - t53664 / 192.0 - 119.0 / 864.0 * t51530 - t53668 / 384.0 - t53671 / 768.0 + t53675 / 4.0;
    (t55321,)
}
