//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 778/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk778(t2259: f64, t6402: f64, t2255: f64, t2279: f64, t6350: f64, t343: f64, t6269: f64, t337: f64, t2121: f64, t2134: f64, t2115: f64, t2142: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6403 = t6402 * t2259;
    let t6406 = t2255 * t6350 * t2279;
    let t6409 = t6269 * t343;
    let t6410 = t337 * t6409;
    let t6411 = t2121 * t6410;
    let t6413 = t2134 * t6411 / 32.0_f64;
    let t6414 = t2115 * t2142;
    (t6403, t6406, t6410, t6411, t6413, t6414)
}
