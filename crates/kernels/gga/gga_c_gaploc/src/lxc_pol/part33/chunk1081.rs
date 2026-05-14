//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1081/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1081<F: Float>(t1897: F, t27348: F, t954: F, t23433: F, t2936: F, t10697: F, t29020: F, t9647: F, t10749: F, t731: F, t23362: F, t5269: F, t10755: F, t5293: F, t27403: F, t10636: F, t5524: F) -> (F, F, F, F, F, F, F, F) {
    let t32477 = 0.76905262301422242837e-2 * t1897 * t954 * t27348;
    let t32480 = 0.23071578690426672851e-1 * t1897 * t2936 * t23433;
    let t32482 = t9647 * t10697 * t29020;
    let t32483 = 0.22430701504581487494e-2 * t32482;
    let t32484 = t731 * t10749;
    let t32485 = 0.85450291446024714264e-3 * t32484;
    let t32488 = 0.46143157380853345702e-1 * t5269 * t2936 * t23362;
    let t32490 = 0.20508069947045931424e-1 * t5293 * t10755;
    let t32493 = 0.15381052460284448567e-1 * t5269 * t954 * t27403;
    let t32511 = 0.8545029144602471425e-3 * t5524 * t10636;
    (t32477, t32480, t32483, t32485, t32488, t32490, t32493, t32511)
}
