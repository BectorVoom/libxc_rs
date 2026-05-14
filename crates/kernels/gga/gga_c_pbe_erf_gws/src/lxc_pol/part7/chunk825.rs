//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 825/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk825<F: Float>(t16978: F, t626: F, t11: F, t625: F, t174: F, t205: F, t2200: F, t1416: F) -> (F, F, F, F, F) {
    let t16979 = t626 * t16978;
    let t16981 = t11 * t625 * t16979;
    let t16984 = t174 * t2200 * t205;
    let t16985 = 0.19591358024691358025e-1 * t16984;
    let t16986 = t1416 * t1416;
    (t16979, t16981, t16984, t16985, t16986)
}
