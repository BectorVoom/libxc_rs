//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2071/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2071(t2132: f64, t24746: f64, t86197: f64, t1170: f64, t2121: f64, t24611: f64, t225: f64, t24871: f64, t2122: f64, t7319: f64, t24574: f64, t24597: f64) -> (f64, f64, f64, f64, f64) {
    let t86368 = t2132 * t86197 * t24746;
    let t86390 = t2121 * t1170 * t24611;
    let t86400 = t24871 * t225;
    let t86403 = t7319 * t2122;
    let t86409 = t24574 * t24597;
    (t86368, t86390, t86400, t86403, t86409)
}
