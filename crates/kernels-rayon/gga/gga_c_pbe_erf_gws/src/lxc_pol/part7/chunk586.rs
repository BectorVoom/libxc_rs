//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 586/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk586(t4422: f64, t824: f64, t822: f64, t833: f64, t2387: f64, t2391: f64, t2242: f64, t941: f64, t2220: f64, t338: f64, t845: f64, t376: f64, t4379: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4423 = t824 * t4422;
    let t4424 = t822 * t4423;
    let t4425 = t4424 * t833;
    let t4427 = t2387 * t2391;
    let t4430 = t2242 * t941;
    let t4433 = t338 * t2220 * t845;
    let t4436 = t376 * t4379;
    (t4423, t4424, t4425, t4427, t4430, t4433, t4436)
}
