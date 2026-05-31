//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1139/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1139<F: Float>(t20412: F, t6648: F, t2105: F, t343: F, t874: F, t2271: F, t6643: F, t822: F, t2118: F, t2382: F, t6491: F, t860: F) -> (F, F, F, F) {
    let t20414 = t20412 * t6648 / F::cast_from(8.0_f64);
    let t20416 = t2105 * t874 * t343;
    let t20421 = t2271 * t6643;
    let t20422 = t822 * t20421;
    let t20424 = t20422 * t6648 / F::cast_from(8.0_f64);
    let t20428 = t2382 * t2118 * t6491 * t860 / F::cast_from(24.0_f64);
    (t20414, t20416, t20424, t20428)
}
