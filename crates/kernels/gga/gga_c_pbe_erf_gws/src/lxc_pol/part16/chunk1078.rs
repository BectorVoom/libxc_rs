//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1078/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1078<F: Float>(t13972: F, t14443: F, t1123: F, t52033: F, t833: F, t850: F, t14423: F, t14682: F, t3989: F, t6360: F, t50998: F, t51066: F, t9650: F, t1162: F, t13917: F, t3223: F, t361: F, t874: F) -> (F, F, F, F, F) {
    let t53011 = t13972 * t14443;
    let t53015 = t850 * t1123 * t52033 * t833;
    let t53019 = t3989 * t14682 * t14423 * t6360;
    let t53038 = t50998 * t51066 * t9650;
    let t53053 = t13917 * t361 * t1162 * t874 * t3223;
    (t53011, t53015, t53019, t53038, t53053)
}
