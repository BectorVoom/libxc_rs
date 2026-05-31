//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 956/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk956<F: Float>(t401: F, t5250: F, t1251: F, t1863: F, t1857: F, t5268: F, t5265: F, t16704: F, t5236: F, t16677: F, t16686: F, t16693: F, t16713: F, t25: F, t5264: F) -> F {
    let t17715 = t401 * t5250;
    let t17720 = t1251 * t1863;
    let t17722 = t1251 * t1857;
    let t17724 = t401 * t5268;
    let t17726 = t401 * t5265;
    let t17728 = F::cast_from(0.37324691358024691357e0_f64) * t16704;
    let t17729 = t401 * t5236;
    let t17734 = -F::cast_from(0.35555555555555555556e-1_f64) * t17715 + F::cast_from(0.35555555555555555554e-1_f64) * t25 * t5264 * t16713 - F::cast_from(0.44444444444444444445e-1_f64) * t17720 - F::cast_from(0.14814814814814814815e-1_f64) * t17722 + F::cast_from(0.17777777777777777778e-1_f64) * t17724 + F::cast_from(0.79012345679012345679e-2_f64) * t17726 + t17728 - F::cast_from(0.10666666666666666667e0_f64) * t17729 + F::cast_from(0.86380000000000000002e0_f64) * t16677 - F::cast_from(0.9597777777777777778e-1_f64) * t16686 - F::cast_from(0.12957e1_f64) * t16693;
    t17734
}
