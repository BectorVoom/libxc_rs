//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1149/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1149<F: Float>(t6203: F, t6347: F, t4379: F, t5: F, t2147: F, t337: F, t2146: F, t6253: F, t6332: F, t19562: F, t346: F, t2124: F, t6800: F) -> (F, F, F, F, F) {
    let t20576 = t6203 * t6347;
    let t20578 = t5 * t4379;
    let t20580 = t2147 * t337 * t20578;
    let t20582 = t2146 * t20580 / F::cast_from(12.0_f64);
    let t20583 = t6253 * t6332;
    let t20584 = F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t20583;
    let t20585 = t19562 * t346;
    let t20588 = t6800 * t20585 * t2124 / F::cast_from(16.0_f64);
    (t20576, t20578, t20582, t20584, t20588)
}
