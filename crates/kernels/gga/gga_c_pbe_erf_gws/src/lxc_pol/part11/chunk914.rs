//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 914/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk914<F: Float>(t12843: F, t401: F, t12527: F, t586: F, t10419: F, t2753: F, t11032: F, t2640: F, t12634: F, t5129: F, t587: F, t12583: F, t626: F, t12766: F, t1620: F, t4934: F) -> (F, F, F, F, F, F, F) {
    let t41976 = t401 * t12843;
    let t42011 = t12527 * t586;
    let t42014 = t10419 * t2753;
    let t42037 = t11032 * t2640;
    let t42050 = t587 * t5129 * t12634;
    let t42094 = t12583 * t626;
    let t42109 = t1620 * t4934 * t12766;
    (t41976, t42011, t42014, t42037, t42050, t42094, t42109)
}
