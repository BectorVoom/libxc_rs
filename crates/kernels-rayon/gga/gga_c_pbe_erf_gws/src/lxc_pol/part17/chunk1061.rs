//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1061/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1061(t874: f64, t3222: f64, t9638: f64, t2323: f64, t3279: f64, t6384: f64, t8939: f64, t904: f64, t3258: f64, t6390: f64, t2255: f64, t2313: f64, t3111: f64, param_a_c: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9639 = t874 * param_a_c;
    let t9640 = t9639 * t3222;
    let t9641 = t9638 * t9640;
    let t9645 = 35.0_f64 / 576.0_f64 * t2323 * t3279;
    let t9647 = t6384 * t904 * t8939;
    let t9650 = t3258 * t6390;
    let t9651 = t2255 * t9650;
    let t9655 = t2255 * t3111 * t2313;
    (t9640, t9641, t9645, t9647, t9650, t9651, t9655)
}
