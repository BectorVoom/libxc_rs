//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 940/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk940<F: Float>(t11618: F, t254: F, t906: F, t369: F, t3772: F, t3848: F, t810: F, t1130: F, t2494: F, t3851: F, t339: F, t9807: F, t11316: F, t2178: F, t2181: F, t3154: F, t3159: F, t3162: F, t340: F, t6424: F, t6429: F, t870: F, t871: F, t9053: F, t9056: F) -> (F, F) {
    let t11700 = t254 * t11618;
    let t11701 = t11700 * t906;
    let t11706 = t3772 * t369;
    let t11717 = t3848 * t810;
    let t11720 = t1130 * t2494;
    let t11725 = t3851 * t810;
    let t11728 = t339 * t9807;
    let t11731 = -t11316 * t339 * t340 + 6.0 * t1130 * t9053 + 3.0 * t11706 * t871 + 60.0 * t11717 * t6429 - 24.0 * t11720 * t2181 - 12.0 * t11725 * t2181 + 3.0 * t11728 * t870 + 3.0 * t2178 * t3851 + 6.0 * t3154 * t3162 - 24.0 * t3159 * t9056 - 12.0 * t3848 * t6424;
    (t11701, t11731)
}
