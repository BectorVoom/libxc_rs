//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1155/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1155<F: Float>(t2915: F, t3057: F, t1102: F, t26164: F, t8700: F, t3053: F, t3058: F, t3061: F, t26184: F, t26188: F, t26192: F, t26200: F, t26203: F, t26206: F, t26209: F, t26212: F, t26220: F, t26222: F) -> (F, F, F, F, F) {
    let t26224 = 1.0 / t3057 / t2915;
    let t26228 = 0.12304676425209353917e5 * t1102 * t26224 * t26164 * t8700;
    let t26229 = t3053 * t3053;
    let t26233 = 0.51947267698127589897e2 * t1102 * t3058 * t26229 * t3061;
    let t26234 = -t26184 - t26188 + t26192 + t26200 - t26203 - t26206 + t26209 + t26212 - t26220 - t26222 + t26228 - t26233;
    (t26224, t26228, t26229, t26233, t26234)
}
