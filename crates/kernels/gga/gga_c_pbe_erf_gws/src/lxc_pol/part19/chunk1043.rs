//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1043/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1043<F: Float>(t1130: F, t2494: F, t3851: F, t810: F, t339: F, t9807: F, t11316: F, t11706: F, t11717: F, t2178: F, t2181: F, t3154: F, t3159: F, t3162: F, t340: F, t3848: F, t6424: F, t6429: F, t870: F, t871: F, t9053: F, t9056: F) -> F {
    let t11720 = t1130 * t2494;
    let t11725 = t3851 * t810;
    let t11728 = t339 * t9807;
    let t11731 = -t11316 * t339 * t340 + F::new(6.0) * t1130 * t9053 + F::new(3.0) * t11706 * t871 + F::new(60.0) * t11717 * t6429 - F::new(24.0) * t11720 * t2181 - F::new(12.0) * t11725 * t2181 + F::new(3.0) * t11728 * t870 + F::new(3.0) * t2178 * t3851 + F::new(6.0) * t3154 * t3162 - F::new(24.0) * t3159 * t9056 - F::new(12.0) * t3848 * t6424;
    t11731
}
