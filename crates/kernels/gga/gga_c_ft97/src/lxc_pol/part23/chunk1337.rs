//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1337/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1337<F: Float>(t113076: F, t19002: F, t1901: F, t113080: F, t19006: F, t10683: F, t24980: F, t5225: F, t6318: F, t856: F, t25162: F, t31605: F, t31602: F, t99312: F, t2862: F, t28741: F, t28746: F) -> (F, F, F, F, F, F, F, F) {
    let t126715 = t1901 * t113076 * t19002;
    let t126718 = t1901 * t113080 * t19006;
    let t126723 = t24980 * t10683 * t6318 * t5225 * t856;
    let t126725 = t25162 * t31605;
    let t126726 = t126725 / 9.0;
    let t126727 = t99312 * t31602;
    let t126728 = t126727 / 3.0;
    let t126731 = t24980 * t2862 * t28746 * t28741;
    (t126715, t126718, t126723, t126725, t126726, t126727, t126728, t126731)
}
