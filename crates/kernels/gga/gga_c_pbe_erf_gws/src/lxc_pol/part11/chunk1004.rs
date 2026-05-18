//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1004/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1004<F: Float>(t12439: F, t1620: F, t5493: F, t12443: F, t12782: F, t5137: F, t639: F, t12486: F, t583: F, t12589: F, t185: F, t582: F) -> (F, F, F, F, F) {
    let t39883 = t1620 * t5493 * t12439;
    let t39886 = t1620 * t5493 * t12443;
    let t39931 = t639 * t5137 * t12782;
    let t39951 = t12486 * t583;
    let t40039 = t185 * t582 * t12589;
    (t39883, t39886, t39931, t39951, t40039)
}
