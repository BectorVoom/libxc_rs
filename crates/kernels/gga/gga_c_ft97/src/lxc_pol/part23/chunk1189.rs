//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1189/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1189<F: Float>(t24237: F, t30883: F, t1403: F, t30920: F, t681: F, t27968: F, t6745: F, t107910: F, t107943: F, t107945: F, t107958: F, t1091: F, t1173: F, t193: F, t2354: F, t2404: F, t27882: F, t27956: F, t28032: F, t28038: F, t30921: F, t3821: F, t5996: F, t6002: F, t6008: F, t683: F, t6838: F, t96782: F) -> (F,) {
    let t121816 = t24237 * t30883;
    let t121819 = t1403 * t681 * t30920;
    let t121821 = t6745 * t27968;
    let t121840 = -2.0 / 3.0 * t1403 * t193 * t27882 * t27956 + 2.0 / 27.0 * t96782 + t5996 * t30921 + t121816 / 27.0 - t121819 / 3.0 + 2.0 / 9.0 * t121821 - t6002 * t2354 * t107910 * t1091 / 9.0 + t107943 + t107945 + 2.0 / 9.0 * t6002 * t683 * t6838 * t28032 - 2.0 / 27.0 * t6002 * t2404 * t6838 * t28038 - t107958 - 2.0 / 3.0 * t1403 * t193 * t6008 * t1173 * t3821;
    (t121840,)
}
