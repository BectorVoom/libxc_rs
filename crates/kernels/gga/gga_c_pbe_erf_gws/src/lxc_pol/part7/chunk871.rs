//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 871/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk871<F: Float>(t401: F, t5250: F, t1251: F, t1863: F, t1857: F, t5268: F, t5265: F, t16704: F, t5236: F, t16677: F, t16686: F, t16693: F, t16713: F, t25: F, t5264: F, t5253: F) -> (F, F) {
    let t17715 = t401 * t5250;
    let t17720 = t1251 * t1863;
    let t17722 = t1251 * t1857;
    let t17724 = t401 * t5268;
    let t17726 = t401 * t5265;
    let t17728 = 0.37324691358024691357e0 * t16704;
    let t17729 = t401 * t5236;
    let t17734 = -0.35555555555555555556e-1 * t17715 + 0.35555555555555555554e-1 * t25 * t5264 * t16713 - 0.44444444444444444445e-1 * t17720 - 0.14814814814814814815e-1 * t17722 + 0.17777777777777777778e-1 * t17724 + 0.79012345679012345679e-2 * t17726 + t17728 - 0.10666666666666666667e0 * t17729 + 0.86380000000000000002e0 * t16677 - 0.9597777777777777778e-1 * t16686 - 0.12957e1 * t16693;
    let t17745 = t401 * t5253;
    (t17734, t17745)
}
