//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 799/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk799(t6621: f64, t860: f64, t2087: f64, t2142: f64, t899: f64, t912: f64, t923: f64) -> (f64, f64, f64) {
    let t6623 = t6621 * t860 / 96.0_f64;
    let t6624 = t2087 * t2142;
    let t6625 = 7.0_f64 / 96.0_f64 * t6624;
    let t6627 = t899 * t912 * t923;
    (t6623, t6625, t6627)
}
