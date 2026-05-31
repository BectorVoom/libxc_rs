//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1088/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1088<F: Float>(t13856: F, t13862: F, t13866: F, t13870: F, t13873: F, t13875: F, t13878: F, t13881: F, t13884: F, t13886: F, t13890: F, t13895: F, t13896: F, t13900: F, t13904: F, t13907: F, t13911: F, t2408: F, t335: F, t6793: F) -> F {
    let t13914 = -t13856 / F::cast_from(48.0_f64) + t13862 / F::cast_from(384.0_f64) + t13866 / F::cast_from(384.0_f64) - t13870 / F::cast_from(3072.0_f64) + t13873 / F::cast_from(48.0_f64) - F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t13875 + t13878 / F::cast_from(768.0_f64) - t335 * t13881 / F::cast_from(96.0_f64) + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t13884 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t13886 - t2408 * t13890 / F::cast_from(12.0_f64) + t13895 + t13896 / F::cast_from(48.0_f64) - t13900 / F::cast_from(3072.0_f64) + t13904 / F::cast_from(1536.0_f64) + t13907 / F::cast_from(1536.0_f64) + t6793 * t13911 / F::cast_from(24.0_f64);
    t13914
}
