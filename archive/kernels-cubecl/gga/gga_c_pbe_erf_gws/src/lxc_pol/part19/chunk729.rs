//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 729/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk729<F: Float>(t1150: F, t4039: F, t1154: F, t4043: F, t1158: F, t4049: F, t1105: F, t1205: F) -> (F, F, F, F) {
    let t4176 = t4039 * t1150;
    let t4178 = t4043 * t1154;
    let t4180 = t4049 * t1158;
    let t4207 = t1205 * t1105;
    (t4176, t4178, t4180, t4207)
}
