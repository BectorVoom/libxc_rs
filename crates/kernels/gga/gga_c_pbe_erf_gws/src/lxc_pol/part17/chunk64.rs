//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 64/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk64<F: Float>(t11: F, t153: F, t156: F, t25: F, t41: F) -> F {
    let t159 = F::new(1.0) + F::new(0.5175e-2) * t11 + F::new(0.204825e-1) * t25 - F::cast_from(0.30486129349252551566e-2_f64) * t41 + F::new(0.3485625e-3) * t153 * t156;
    t159
}
