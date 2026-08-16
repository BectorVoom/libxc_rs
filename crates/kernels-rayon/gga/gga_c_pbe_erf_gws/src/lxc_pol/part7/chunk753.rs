//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 753/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk753(t343: f64, t6177: f64, t337: f64, t2121: f64, t2134: f64, t2365: f64, t828: f64) -> (f64, f64, f64, f64) {
    let t6178 = t6177 * t343;
    let t6179 = t337 * t6178;
    let t6180 = t2121 * t6179;
    let t6182 = t2134 * t6180 / 32.0_f64;
    let t6183 = t2365 * t828;
    (t6179, t6180, t6182, t6183)
}
