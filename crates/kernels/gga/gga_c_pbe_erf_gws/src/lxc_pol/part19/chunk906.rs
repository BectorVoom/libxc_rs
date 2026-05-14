//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 906/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk906<F: Float>(t10201: F, t225: F, t3459: F, t679: F, t230: F, t11009: F, t11014: F, t11016: F, t11018: F, t11021: F, t11024: F, t11027: F, t11031: F, t11034: F, t231: F, t7873: F, t7876: F, t7880: F, t7890: F, t7905: F) -> (F,) {
    let t11226 = t10201 * t225;
    let t11229 = t3459 * t679;
    let t11231 = t3459 * t230;
    let t11233 = -t7873 - t7876 + t7880 + t7890 + t11009 + t11014 + t11016 - t7905 - t11018 + 4.0 / 3.0 * t11226 * t231 + 4.0 / 3.0 * t11229 + t11021 - t11024 - t11027 + 4.0 / 3.0 * t11231 + t11031 + t11034;
    (t11233,)
}
