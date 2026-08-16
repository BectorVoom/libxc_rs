//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1325/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1325<F: Float>(t1205: F, t20173: F, t53645: F, t14918: F, t2367: F, t1115: F, t14185: F, t14321: F, t2408: F, t3066: F, t35566: F, t51530: F, t52350: F, t52353: F, t53631: F, t53639: F, t53643: F, t53664: F, t53668: F, t53671: F, t53675: F, t9283: F, t9297: F, t9702: F) -> F {
    let t55297 = t20173 * t1205;
    let t55311 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t53645;
    let t55315 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t2367 * t14918;
    let t55321 = -F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t52353 - t53631 / F::cast_from(192.0_f64) + t3066 * t9283 * t55297 * t9297 / F::cast_from(4.0_f64) + t53639 / F::cast_from(1536.0_f64) - t2408 * t35566 * t14321 / F::cast_from(12.0_f64) - t2408 * t9283 * t14185 * t9702 / F::cast_from(12.0_f64) + t53643 / F::cast_from(768.0_f64) - t55311 - t1115 * t52350 / F::cast_from(96.0_f64) + t55315 - t53664 / F::cast_from(192.0_f64) - F::cast_from(119.0_f64) / F::cast_from(864.0_f64) * t51530 - t53668 / F::cast_from(384.0_f64) - t53671 / F::cast_from(768.0_f64) + t53675 / F::cast_from(4.0_f64);
    t55321
}
