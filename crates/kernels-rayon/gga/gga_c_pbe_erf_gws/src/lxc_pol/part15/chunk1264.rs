//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1264/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1264(t1114: f64, t50935: f64, t13793: f64, t1112: f64, t2306: f64, t3074: f64, t833: f64, t837: f64, t14657: f64, t51721: f64, t13984: f64, t13972: f64, t14799: f64) -> (f64, f64, f64, f64, f64) {
    let t53571 = t1114 * t50935;
    let t53572 = t53571 * t13793;
    let t53577 = t3074 * t2306 * t1112 * t837 * t833;
    let t53578 = 7.0_f64 / 144.0_f64 * t53577;
    let t53579 = t14657 * t51721;
    let t53581 = t53571 * t13984;
    let t53583 = t13972 * t14799;
    (t53572, t53578, t53579, t53581, t53583)
}
