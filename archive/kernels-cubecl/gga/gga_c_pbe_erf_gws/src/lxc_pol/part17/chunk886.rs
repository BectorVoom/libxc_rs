//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 886/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk886<F: Float>(t587: F, t7680: F, t5359: F, t7634: F, t7636: F, t7637: F, t7639: F, t7644: F, t7648: F, t7650: F, t7655: F, t7658: F, t7662: F, t7665: F, t7668: F, t7672: F, t7676: F, t7679: F) -> (F, F) {
    let t7682 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t587 * t7680;
    let t7683 = t7634 - t7636 - t7637 + t5359 + t7639 - t7644 + t7648 + t7650 - t7655 + t7658 + t7662 - t7665 - t7668 + t7672 - t7676 - t7679 - t7682;
    (t7682, t7683)
}
