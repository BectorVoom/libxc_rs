//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1016/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1016<F: Float>(t1158: F, t14058: F, t14015: F, t3287: F, t1140: F, t4033: F, t2080: F, t3260: F, t332: F, t346: F, t859: F, t1135: F, t3065: F, t2134: F, t1161: F, t3222: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14558 = t14058 * t1158;
    let t14560 = t14015 * t3287;
    let t14563 = t4033 * t1140;
    let t14565 = t2080 * t3260;
    let t14567 = t346 * t332 * t859;
    let t14568 = t14565 * t14567;
    let t14570 = t3065 * t1135;
    let t14571 = t2134 * t14570;
    let t14582 = t1161 * param_a_c;
    let t14583 = t14582 * t3222;
    (t14558, t14560, t14563, t14565, t14567, t14568, t14570, t14571, t14583)
}
