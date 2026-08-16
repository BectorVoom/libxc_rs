//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 468/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk468(t2482: f64, t27: f64, t823: f64, t136: f64, t826: f64, t221: f64, t837: f64, t737: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2484 = t2482 * t823 * t27;
    let t2485 = t826 * t136;
    let t2487 = t2485 * t221 * t837;
    let t2488 = t2484 * t2487;
    let t2490 = t737 * t737;
    let t2491 = 1.0_f64 / t2490;
    (t2484, t2485, t2487, t2488, t2490, t2491)
}
