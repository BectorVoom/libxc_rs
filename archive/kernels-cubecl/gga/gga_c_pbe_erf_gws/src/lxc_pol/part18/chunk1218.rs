//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1218/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1218<F: Float>(t2250: F, t51350: F, t2153: F, t899: F, t923: F, t56: F, t837: F, t863: F, t911: F, t2331: F, t918: F, t864: F) -> (F, F, F, F, F, F) {
    let t51351 = t2250 * t51350;
    let t51371 = t899 * t2153 * t923;
    let t51382 = t863 * t911 * t837 * t56;
    let t51387 = t899 * t2153 * t2331;
    let t51388 = t51387 * t918;
    let t51395 = t899 * t864 * t2331;
    (t51351, t51371, t51382, t51387, t51388, t51395)
}
