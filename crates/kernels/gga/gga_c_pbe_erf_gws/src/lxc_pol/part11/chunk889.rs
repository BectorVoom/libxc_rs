//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 889/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk889<F: Float>(t5399: F, t56: F, t1641: F, t174: F, t205: F, t2200: F, t1696: F, t5217: F, t735: F, t5292: F, t9: F, t1663: F, t187: F, t22: F) -> (F, F, F, F, F, F, F, F) {
    let t16970 = t56 * t5399;
    let t16971 = t1641 * t1641;
    let t16972 = F::cast_from(1.0_f64) / t16971;
    let t16984 = t174 * t2200 * t205;
    let t16985 = F::cast_from(0.19591358024691358025e-1_f64) * t16984;
    let t17037 = F::cast_from(1.0_f64) / t1641 / t1696;
    let t17139 = t5217 * t735;
    let t17172 = t9 * t5292;
    let t17182 = t22 / t187 / t1663;
    (t16970, t16972, t16984, t16985, t17037, t17139, t17172, t17182)
}
