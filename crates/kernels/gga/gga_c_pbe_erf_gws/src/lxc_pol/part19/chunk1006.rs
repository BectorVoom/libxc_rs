//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1006/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1006<F: Float>(t2367: F, t4083: F, t14072: F, t14084: F, t4094: F, t840: F, t13894: F, t1208: F, t2242: F, t4090: F, t4414: F, t1205: F, t6781: F, t829: F, t830: F) -> (F, F, F, F, F, F, F, F) {
    let t14198 = t2367 * t4083;
    let t14229 = 119.0 / 3456.0 * t14072;
    let t14233 = 35.0 / 216.0 * t14084;
    let t14283 = t840 * t4094;
    let t14295 = 119.0 / 6912.0 * t13894;
    let t14302 = 35.0 / 432.0 * t2242 * t1208;
    let t14305 = t4414 * t4090;
    let t14309 = t6781 * t1205;
    let t14311 = t829 * t830 * t14309;
    (t14198, t14229, t14233, t14283, t14295, t14302, t14305, t14311)
}
