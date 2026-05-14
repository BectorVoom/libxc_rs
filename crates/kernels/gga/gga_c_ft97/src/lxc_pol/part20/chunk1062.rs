//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1062/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1062<F: Float>(t24437: F, t24546: F, t2574: F, t27814: F, t3821: F, t6119: F, t747: F, t3938: F, t713: F, t24477: F, t27787: F, t24543: F, t27789: F, t6896: F, t8232: F, t2360: F, t6837: F) -> (F, F, F, F, F, F, F, F, F) {
    let t108126 = t24437 * t2574 * t24546 * t27814;
    let t108130 = t24437 * t2574 * t6119 * t3821 * t747;
    let t108134 = t24437 * t2574 * t6119 * t3938 * t713;
    let t108137 = t24437 * t2574 * t27787 * t24477;
    let t108138 = t24543 * t27789;
    let t108139 = t108138 / 9.0;
    let t108140 = t8232 * t6896;
    let t108141 = 4.0 / 27.0 * t108140;
    let t108142 = t6837 * t2360;
    (t108126, t108130, t108134, t108137, t108138, t108139, t108140, t108141, t108142)
}
