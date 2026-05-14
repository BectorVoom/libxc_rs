//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 960/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk960<F: Float>(t9176: F, t1133: F, t745: F, t343: F, t1123: F, t2255: F, t1076: F, t874: F, t274: F, t3165: F, t11964: F, t254: F, t906: F, t11819: F, t8599: F, t2168: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12005 = 35.0 / 108.0 * t9176;
    let t12006 = t745 * t1133;
    let t12007 = t12006 * t343;
    let t12008 = t1123 * t12007;
    let t12009 = t2255 * t12008;
    let t12013 = t1076 * t874 * t343;
    let t12014 = t1123 * t12013;
    let t12015 = t2255 * t12014;
    let t12019 = t274 * t3165 * t343;
    let t12020 = t1123 * t12019;
    let t12021 = t2255 * t12020;
    let t12024 = t254 * t11964;
    let t12025 = t12024 * t906;
    let t12029 = t8599 * t11819;
    let t12031 = t2168 * t12029 / 8.0;
    (t12005, t12008, t12009, t12014, t12015, t12020, t12021, t12025, t12031)
}
