//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 955/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk955<F: Float>(t5945: F, t7639: F, t7644: F, t7648: F, t7650: F, t7655: F, t7658: F, t7662: F, t7665: F, t7668: F, t7672: F, t7676: F, t7679: F, t7682: F, t7689: F, t7693: F) -> F {
    let t8446 = t7639 - t7644 + t7648 + t7650 - t7655 + t7658 + t7662 - t7665 - t7668 + t7672 - t7676 - t7679 - t7682 + t7689 + t7693 + F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t5945;
    t8446
}
