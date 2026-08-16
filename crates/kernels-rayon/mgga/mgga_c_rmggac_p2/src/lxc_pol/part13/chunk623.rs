//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 623/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk623(t2205: f64, t2604: f64, t2244: f64, t275: f64, t2262: f64, t504: f64, t699: f64, t798: f64, t903: f64, t2211: f64, t4048: f64, t739: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8206 = t2604 * t2205;
    let t8207 = 0.11974241701863808564e0_f64 * t8206;
    let t8208 = t275 * t2244;
    let t8210 = t504 * t2262;
    let t8211 = 0.39914139006212695214e-1_f64 * t8210;
    let t8212 = t699 * t798;
    let t8213 = t903 * t8212;
    let t8214 = 0.35922725105591425692e0_f64 * t8213;
    let t8215 = t2211 * t4048;
    let t8216 = t739 * t8215;
    (t8207, t8208, t8211, t8212, t8214, t8215, t8216)
}
