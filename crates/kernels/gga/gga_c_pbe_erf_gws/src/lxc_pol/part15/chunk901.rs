//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 901/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk901<F: Float>(t3138: F, t8921: F, t3123: F, t6411: F, t3184: F, t6484: F, t1114: F, t6701: F, t2124: F, t3128: F, t6563: F, t3106: F, t6472: F, t8782: F, t860: F, t3116: F, t6707: F) -> (F, F, F, F, F, F, F) {
    let t8923 = t3138 * t8921 / 48.0;
    let t8925 = t3123 * t6411 / 96.0;
    let t8927 = 7.0 / 72.0 * t6484 * t3184;
    let t8928 = t1114 * t6701;
    let t8930 = t8928 * t2124 / 48.0;
    let t8932 = t3128 * t6563 / 16.0;
    let t8933 = t6472 * t3106;
    let t8934 = t8782 * t8933;
    let t8936 = t8934 * t860 / 96.0;
    let t8938 = t3116 * t6707 / 96.0;
    (t8923, t8925, t8927, t8930, t8932, t8936, t8938)
}
