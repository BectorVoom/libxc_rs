//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1088/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1088<F: Float>(t13856: F, t13862: F, t13866: F, t13870: F, t13873: F, t13875: F, t13878: F, t13881: F, t13884: F, t13886: F, t13890: F, t13895: F, t13896: F, t13900: F, t13904: F, t13907: F, t13911: F, t2408: F, t335: F, t6793: F) -> F {
    let t13914 = -t13856 / F::new(48.0) + t13862 / F::new(384.0) + t13866 / F::new(384.0) - t13870 / F::new(3072.0) + t13873 / F::new(48.0) - F::new(7.0) / F::new(72.0) * t13875 + t13878 / F::new(768.0) - t335 * t13881 / F::new(96.0) + F::new(7.0) / F::new(144.0) * t13884 + F::new(7.0) / F::new(144.0) * t13886 - t2408 * t13890 / F::new(12.0) + t13895 + t13896 / F::new(48.0) - t13900 / F::new(3072.0) + t13904 / F::new(1536.0) + t13907 / F::new(1536.0) + t6793 * t13911 / F::new(24.0);
    t13914
}
