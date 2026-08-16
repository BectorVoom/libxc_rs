//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 774/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk774(t1368: f64, t285: f64, t535: f64, t281: f64, t147: f64, t4576: f64, t131: f64, t2029: f64, t137: f64, t510: f64, t142: f64, t1570: f64) -> (f64, f64, f64, f64) {
    let t5611 = t535 * t1368 * t285;
    let t5612 = t281 * t5611;
    let t5615 = t147 * t4576 * t285;
    let t5617 = 0.11974234010254609094e-1_f64 * t281 * t5615;
    let t5621 = 1.0_f64 / t2029 / t131;
    let t5622 = t5621 * t137;
    let t5623 = t510 * t510;
    let t5624 = t142 * t5623;
    let t5625 = t5622 * t5624;
    let t5628 = t142 * t1570;
    (t5612, t5617, t5625, t5628)
}
