//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1053/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1053<F: Float>(t11776: F, t3139: F, t4028: F, t3871: F, t4049: F, t3875: F, t11785: F, t14101: F, t1184: F, t3799: F, t3867: F, t3805: F, t4023: F, t14031: F, t3765: F, t3810: F, t4039: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t15248 = t3139 * t11776;
    let t15249 = t4028 * t15248;
    let t15251 = t4049 * t3871;
    let t15253 = t4049 * t3875;
    let t15255 = t3139 * t11785;
    let t15256 = t14101 * t15255;
    let t15258 = t1184 * t3799;
    let t15260 = t1184 * t3867;
    let t15262 = t3805 * t4023;
    let t15264 = t14031 * t3765;
    let t15266 = t4039 * t3810;
    (t15248, t15249, t15251, t15253, t15255, t15256, t15258, t15260, t15262, t15264, t15266)
}
