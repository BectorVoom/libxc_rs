//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1193/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1193<F: Float>(t14102: F, t51382: F, t2153: F, t2331: F, t899: F, t918: F, t864: F, t935: F, t14058: F, t2302: F, t1477: F, t360: F, t56: F, t863: F) -> (F, F, F, F, F, F, F) {
    let t51383 = t51382 * t14102;
    let t51387 = t899 * t2153 * t2331;
    let t51388 = t51387 * t918;
    let t51395 = t899 * t864 * t2331;
    let t51396 = t51395 * t935;
    let t51401 = t14058 * t2302;
    let t51407 = t863 * t360 * t1477 * t56;
    (t51383, t51387, t51388, t51395, t51396, t51401, t51407)
}
