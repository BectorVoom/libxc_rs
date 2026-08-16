//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1269/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1269(t53645: f64, t14803: f64, t4414: f64, t13888: f64, t13925: f64, t22343: f64, t2408: f64, t2410: f64, t4002: f64, t51526: f64, t53614: f64, t53617: f64, t53623: f64, t53626: f64, t53629: f64, t53631: f64, t53636: f64, t53639: f64, t53643: f64, t6793: f64, t8754: f64, t9265: f64, t9283: f64) -> f64 {
    let t53646 = 7.0_f64 / 144.0_f64 * t53645;
    let t53656 = 7.0_f64 / 36.0_f64 * t4414 * t14803;
    let t53657 = t6793 * t53617 / 24.0_f64 + t22343 * t13925 / 48.0_f64 - t53623 / 1536.0_f64 + t53626 + 7.0_f64 / 2304.0_f64 * t51526 + t53629 - t53631 / 384.0_f64 - t9265 * t4002 / 96.0_f64 + t53636 + t53639 / 3072.0_f64 + t53643 / 1536.0_f64 - t53646 - t2408 * t9283 * t13888 * t8754 / 12.0_f64 - t2408 * t9283 * t53614 * t2410 / 12.0_f64 + t53656;
    t53657
}
