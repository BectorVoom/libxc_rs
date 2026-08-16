//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 800/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk800(t5: f64, t745: f64, t337: f64, t2121: f64, t810: f64, t816: f64, t274: f64, t1: f64, t2298: f64, t253: f64, t320: f64, t368: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6340 = t5 * t745;
    let t6341 = t337 * t6340;
    let t6342 = t2121 * t6341;
    let t6345 = t816 * t810;
    let t6355 = t745 * t274;
    let t6365 = t2298 * t1;
    let t6366 = t6365 * t253;
    let t6382 = 1.0_f64 / t368 / t320;
    (t6341, t6342, t6345, t6355, t6365, t6366, t6382)
}
