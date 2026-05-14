//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 796/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk796<F: Float>(t2146: F, t8833: F, t3165: F, t5: F, t2142: F, t3108: F, t3106: F, t4395: F, t3074: F, t2118: F, t3178: F, t810: F, t337: F, t6560: F, t814: F, t2147: F) -> (F, F, F, F, F, F, F, F) {
    let t8835 = 7.0 / 72.0 * t2146 * t8833;
    let t8840 = t5 * t3165;
    let t8846 = 7.0 / 144.0 * t3108 * t2142;
    let t8847 = t4395 * t3106;
    let t8848 = t3074 * t8847;
    let t8860 = t2118 * t3106;
    let t8867 = t3178 * t810;
    let t8868 = t337 * t8867;
    let t8869 = t6560 * t8868;
    let t8873 = t337 * t3178 * t814;
    let t8874 = t2147 * t8873;
    (t8835, t8840, t8846, t8848, t8860, t8867, t8869, t8874)
}
