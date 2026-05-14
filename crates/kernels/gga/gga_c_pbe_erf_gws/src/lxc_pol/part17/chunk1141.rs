//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1141/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1141<F: Float>(t14024: F, t3113: F, t14498: F, t9675: F, t9494: F, t4023: F, t9185: F, t14011: F, t9681: F, t14015: F, t9527: F, t51312: F, t9035: F, t14570: F, t6538: F, t3123: F, t51430: F) -> (F, F, F, F, F, F, F, F, F) {
    let t54135 = t3113 * t14024;
    let t54136 = 7.0 / 144.0 * t54135;
    let t54137 = t14498 * t9675;
    let t54139 = t14498 * t9494;
    let t54142 = t9185 * t4023;
    let t54144 = t14011 * t9681;
    let t54146 = t14015 * t9527;
    let t54148 = t9035 * t51312;
    let t54150 = t6538 * t14570;
    let t54152 = t3123 * t51430;
    (t54136, t54137, t54139, t54142, t54144, t54146, t54148, t54150, t54152)
}
