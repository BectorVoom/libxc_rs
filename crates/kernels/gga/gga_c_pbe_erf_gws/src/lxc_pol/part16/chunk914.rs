//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 914/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk914<F: Float>(t9043: F, t9044: F, t9046: F, t9048: F, t3028: F, t369: F, t1109: F, t931: F, t1130: F, t2182: F, t3162: F, t810: F, t2074: F, t339: F, t8574: F, t2178: F, t2181: F, t2183: F, t2186: F, t3154: F, t3159: F, t340: F, t6421: F, t6424: F, t6429: F, t870: F, t871: F) -> (F, F) {
    let t9050 = t9043 + t9044 + t9046 + t9048;
    let t9053 = t3028 * t369;
    let t9056 = t1109 * t931;
    let t9067 = t1130 * t2182;
    let t9070 = t3162 * t810;
    let t9073 = t1130 * t2074;
    let t9076 = t339 * t8574;
    let t9079 = -t339 * t340 * t9050 + 3.0 * t1130 * t6421 + 6.0 * t2178 * t3162 - 24.0 * t2181 * t9070 - 12.0 * t2181 * t9073 - 12.0 * t2183 * t9056 + 3.0 * t2186 * t3154 - 24.0 * t3159 * t6424 + 60.0 * t6429 * t9067 + 3.0 * t870 * t9076 + 6.0 * t871 * t9053;
    (t9050, t9079)
}
