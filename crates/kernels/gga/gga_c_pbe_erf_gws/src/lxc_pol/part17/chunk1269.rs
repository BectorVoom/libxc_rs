//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1269/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1269<F: Float>(t53645: F, t14803: F, t4414: F, t13888: F, t13925: F, t22343: F, t2408: F, t2410: F, t4002: F, t51526: F, t53614: F, t53617: F, t53623: F, t53626: F, t53629: F, t53631: F, t53636: F, t53639: F, t53643: F, t6793: F, t8754: F, t9265: F, t9283: F) -> F {
    let t53646 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t53645;
    let t53656 = F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t4414 * t14803;
    let t53657 = t6793 * t53617 / F::cast_from(24.0_f64) + t22343 * t13925 / F::cast_from(48.0_f64) - t53623 / F::cast_from(1536.0_f64) + t53626 + F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t51526 + t53629 - t53631 / F::cast_from(384.0_f64) - t9265 * t4002 / F::cast_from(96.0_f64) + t53636 + t53639 / F::cast_from(3072.0_f64) + t53643 / F::cast_from(1536.0_f64) - t53646 - t2408 * t9283 * t13888 * t8754 / F::cast_from(12.0_f64) - t2408 * t9283 * t53614 * t2410 / F::cast_from(12.0_f64) + t53656;
    t53657
}
