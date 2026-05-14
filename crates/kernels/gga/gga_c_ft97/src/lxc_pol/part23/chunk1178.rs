//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1178/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1178<F: Float>(t6386: F, t668: F, t1882: F, t29253: F, t29317: F, t29107: F, t8392: F, t29055: F, t56110: F, t113060: F, t113105: F, t113168: F, t113176: F, t113195: F, t113201: F, t113226: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t114222 = t6386 * t668;
    let t114238 = 4.0 / 9.0 * t1882 * t29253;
    let t114244 = 2.0 / 9.0 * t1882 * t29317;
    let t114247 = 2.0 / 27.0 * t8392 * t29107;
    let t114271 = t56110 * t29055;
    let t114282 = 2.0 / 27.0 * t113060;
    let t114292 = 2.0 / 3.0 * t113105;
    let t114312 = 2.0 / 9.0 * t113168;
    let t114314 = 2.0 / 9.0 * t113176;
    let t114318 = t113195 / 54.0;
    let t114320 = t113201 / 54.0;
    let t114328 = t113226 / 9.0;
    (t114222, t114238, t114244, t114247, t114271, t114282, t114292, t114312, t114314, t114318, t114320, t114328)
}
