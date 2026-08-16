//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 741/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk741(t1477: f64, t328: f64, t824: f64, t822: f64, t833: f64, t2242: f64, t941: f64, t2200: f64, t329: f64, t340: f64, t847: f64, t2306: f64, t2365: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4422 = t328 * t1477;
    let t4423 = t824 * t4422;
    let t4424 = t822 * t4423;
    let t4425 = t4424 * t833;
    let t4430 = t2242 * t941;
    let t4442 = t329 * t2200 * t340;
    let t4443 = t4442 * t847;
    let t4473 = t2306 * t2365;
    (t4423, t4424, t4425, t4430, t4442, t4443, t4473)
}
