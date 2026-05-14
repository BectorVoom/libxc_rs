//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 800/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk800<F: Float>(t1051: F, t1084: F, t474: F, t2704: F, t2710: F, t1125: F, t402: F, t156: F, t2942: F, t2948: F, t2707: F, t2737: F, t2698: F, t2987: F, t2701: F, t1055: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t8266 = 0.043374323531126094 * t1084 * t474 * t1051;
    let t8267 = t2704 * t2710;
    let t8271 = 0.06747116993730726 * t1084 * t1125 * t402;
    let t8274 = 0.1301229705933783 * t1084 * t156 * t2942;
    let t8277 = 3.8527556876111295 * t1084 * t156 * t2948;
    let t8278 = t2704 * t2707;
    let t8285 = 0.021687161765563047 * t1084 * t156 * t2737;
    let t8286 = t2704 * t2698;
    let t8290 = 38.02486811957057 * t1084 * t156 * t2987;
    let t8291 = t2704 * t2701;
    let t8296 = 1.2842518958703766 * t1084 * t474 * t1055;
    (t8266, t8267, t8271, t8274, t8277, t8278, t8285, t8286, t8290, t8291, t8296)
}
