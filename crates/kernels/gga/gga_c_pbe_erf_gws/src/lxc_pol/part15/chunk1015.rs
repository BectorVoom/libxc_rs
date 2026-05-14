//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1015/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1015<F: Float>(t3268: F, t4049: F, t1158: F, t14058: F, t14015: F, t3287: F, t1140: F, t4033: F, t2080: F, t3260: F, t332: F, t346: F, t859: F) -> (F, F, F, F, F, F) {
    let t14556 = t4049 * t3268;
    let t14558 = t14058 * t1158;
    let t14560 = t14015 * t3287;
    let t14563 = t4033 * t1140;
    let t14565 = t2080 * t3260;
    let t14567 = t346 * t332 * t859;
    (t14556, t14558, t14560, t14563, t14565, t14567)
}
