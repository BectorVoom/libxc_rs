//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1023/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1023<F: Float>(t12840: F, t401: F, t12846: F, t12855: F, t12858: F, t12837: F, t12843: F, t12527: F, t586: F, t10419: F, t2753: F, t11032: F, t2640: F) -> (F, F, F, F, F, F, F, F, F) {
    let t41888 = t401 * t12840;
    let t41890 = t401 * t12846;
    let t41939 = t401 * t12855;
    let t41941 = t401 * t12858;
    let t41974 = t401 * t12837;
    let t41976 = t401 * t12843;
    let t42011 = t12527 * t586;
    let t42014 = t10419 * t2753;
    let t42037 = t11032 * t2640;
    (t41888, t41890, t41939, t41941, t41974, t41976, t42011, t42014, t42037)
}
