//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1267/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1267<F: Float>(t46251: F, t28074: F, t21328: F, t50002: F, t858: F, t884: F, t11600: F, t11808: F, t50019: F, t866: F, t867: F, t46324: F) -> (F, F, F, F, F, F) {
    let t50187 = F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t46251;
    let t50189 = F::cast_from(455.0_f64) / F::cast_from(162.0_f64) * t28074;
    let t50193 = F::cast_from(5.0_f64) / F::cast_from(4.0_f64) * t884 * t21328 * t858 * t50002;
    let t50201 = t11600 * t11808 / F::cast_from(8.0_f64);
    let t50206 = t866 * t867 * t858 * t50019 / F::cast_from(96.0_f64);
    let t50207 = F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t46324;
    (t50187, t50189, t50193, t50201, t50206, t50207)
}
