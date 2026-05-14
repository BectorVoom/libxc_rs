//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 989/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk989<F: Float>(t14136: F, t14138: F, t1173: F, t2222: F, t4116: F, t945: F, t2182: F, t4066: F, t810: F, t2074: F, t1206: F, t353: F, t4386: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14139 = t14136 * t14138;
    let t14141 = t1173 * t2222;
    let t14161 = t4116 * t945;
    let t14166 = t4066 * t2182;
    let t14169 = t14161 * t810;
    let t14175 = t4066 * t2074;
    let t14180 = t1206 * t810;
    let t14181 = t353 * t14180;
    let t14182 = t4386 * t14181;
    (t14139, t14141, t14161, t14166, t14169, t14175, t14180, t14181, t14182)
}
