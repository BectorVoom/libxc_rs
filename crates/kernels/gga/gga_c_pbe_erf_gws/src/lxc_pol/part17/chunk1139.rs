//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1139/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1139<F: Float>(t14046: F, t4171: F, t3268: F, t4049: F, t1158: F, t14058: F, t14015: F, t3287: F, t1140: F, t4033: F, t2080: F, t3260: F) -> (F, F, F, F, F, F) {
    let t14554 = t14046 * t4171;
    let t14556 = t4049 * t3268;
    let t14558 = t14058 * t1158;
    let t14560 = t14015 * t3287;
    let t14563 = t4033 * t1140;
    let t14565 = t2080 * t3260;
    (t14554, t14556, t14558, t14560, t14563, t14565)
}
