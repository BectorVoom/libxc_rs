//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 989/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk989<F: Float>(t2397: F, t4424: F, t2387: F, t4423: F, t833: F, t2233: F, t4442: F, t4414: F, t4493: F, t2246: F, t4433: F, t6757: F, t2306: F, t4383: F, t2382: F, t4395: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19875 = t4424 * t2397;
    let t19878 = t2387 * t4423 * t833;
    let t19880 = t4442 * t2233;
    let t19888 = t4414 * t4493;
    let t19890 = t2246 * t4433;
    let t19892 = t4414 * t6757;
    let t19894 = t2306 * t4383;
    let t19895 = t2382 * t19894;
    let t19898 = t4395 * t4383;
    (t19875, t19878, t19880, t19888, t19890, t19892, t19894, t19895, t19898)
}
