//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1160/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1160<F: Float>(t2289: F, t6497: F, t2168: F, t2195: F, t3139: F, t6269: F, t2156: F, t2157: F, t2155: F, t858: F, t867: F, t2251: F, t2300: F) -> (F, F, F, F, F, F) {
    let t20720 = t2289 * t6497;
    let t20725 = t2168 * t3139 * t6269 * t2195 / F::cast_from(16.0_f64);
    let t20726 = t2156 * t2156;
    let t20727 = t20726 * t2157;
    let t20731 = F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t2155 * t867 * t858 * t20727;
    let t20732 = t2251 * t2300;
    (t20720, t20725, t20726, t20727, t20731, t20732)
}
