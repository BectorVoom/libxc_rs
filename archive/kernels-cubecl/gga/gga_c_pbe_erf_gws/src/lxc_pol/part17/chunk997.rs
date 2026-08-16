//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 997/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk997<F: Float>(t6659: F, t858: F, t8939: F, t884: F, t2079: F, t2112: F, t326: F, t3107: F, t860: F, t2119: F, t3039: F, t2124: F) -> (F, F, F, F) {
    let t8941 = t6659 * t858 * t8939;
    let t8943 = t884 * t8941 / F::cast_from(4.0_f64);
    let t8944 = t2079 * t2112;
    let t8945 = t326 * t8944;
    let t8946 = t8945 * t3107;
    let t8948 = t8946 * t860 / F::cast_from(96.0_f64);
    let t8949 = t3039 * t2119;
    let t8951 = t8949 * t2124 / F::cast_from(48.0_f64);
    (t8943, t8945, t8948, t8951)
}
