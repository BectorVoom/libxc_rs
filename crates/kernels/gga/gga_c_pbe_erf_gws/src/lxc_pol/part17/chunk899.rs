//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 899/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk899<F: Float>(t6672: F, t8897: F, t2142: F, t3120: F, t332: F, t6238: F, t863: F, t2156: F, t6241: F, t3131: F, t3139: F, t2158: F, t3138: F, t3037: F, t339: F, t2306: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8899 = t6672 * t8897 / 24.0;
    let t8901 = 7.0 / 144.0 * t3120 * t2142;
    let t8903 = t863 * t6238 * t332;
    let t8904 = t6241 * t2156;
    let t8906 = t3139 * t3131 * t8904;
    let t8908 = t8903 * t8906 / 16.0;
    let t8910 = t3139 * t3131 * t2158;
    let t8912 = t3138 * t8910 / 16.0;
    let t8913 = t3037 * t339;
    let t8914 = t2306 * t8913;
    (t8899, t8901, t8904, t8906, t8908, t8910, t8912, t8913, t8914)
}
