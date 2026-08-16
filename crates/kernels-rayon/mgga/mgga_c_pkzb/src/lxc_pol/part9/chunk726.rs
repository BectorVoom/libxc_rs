//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 726/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk726(t1586: f64, t465: f64, t148: f64, t519: f64, t1503: f64, t534: f64, t471: f64, t204: f64, t492: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5044 = t465 * t1586;
    let t5048 = t148 * t519;
    let t5052 = t465 * t1503;
    let t5056 = t148 * t534;
    let t5063 = t148 * t471;
    let t5066 = 0.71233333333333333332e-1_f64 * t204 * t5063 * t492;
    (t5044, t5048, t5052, t5056, t5063, t5066)
}
