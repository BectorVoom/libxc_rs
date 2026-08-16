//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1043/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1043(t1130: f64, t2494: f64, t3851: f64, t810: f64, t339: f64, t9807: f64, t11316: f64, t11706: f64, t11717: f64, t2178: f64, t2181: f64, t3154: f64, t3159: f64, t3162: f64, t340: f64, t3848: f64, t6424: f64, t6429: f64, t870: f64, t871: f64, t9053: f64, t9056: f64) -> f64 {
    let t11720 = t1130 * t2494;
    let t11725 = t3851 * t810;
    let t11728 = t339 * t9807;
    let t11731 = -t11316 * t339 * t340 + 6.0_f64 * t1130 * t9053 + 3.0_f64 * t11706 * t871 + 60.0_f64 * t11717 * t6429 - 24.0_f64 * t11720 * t2181 - 12.0_f64 * t11725 * t2181 + 3.0_f64 * t11728 * t870 + 3.0_f64 * t2178 * t3851 + 6.0_f64 * t3154 * t3162 - 24.0_f64 * t3159 * t9056 - 12.0_f64 * t3848 * t6424;
    t11731
}
