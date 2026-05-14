//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 896/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk896<F: Float>(t3138: F, t8842: F, t2142: F, t3108: F, t3106: F, t4395: F, t3074: F, t814: F, t857: F, t858: F, t856: F, t6229: F, t2170: F, t2171: F, t8840: F, t2168: F) -> (F, F, F, F, F, F, F) {
    let t8844 = t3138 * t8842 / 24.0;
    let t8846 = 7.0 / 144.0 * t3108 * t2142;
    let t8847 = t4395 * t3106;
    let t8848 = t3074 * t8847;
    let t8850 = t857 * t858 * t814;
    let t8851 = t856 * t8850;
    let t8853 = t8848 * t8851 / 32.0;
    let t8854 = 35.0 / 216.0 * t6229;
    let t8856 = t2170 * t8840 * t2171;
    let t8858 = t2168 * t8856 / 24.0;
    (t8844, t8846, t8848, t8853, t8854, t8856, t8858)
}
