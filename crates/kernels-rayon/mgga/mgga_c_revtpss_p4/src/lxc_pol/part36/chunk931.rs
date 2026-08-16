//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 931/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk931(t1811: f64, t5219: f64, t1284: f64, t6564: f64, t3302: f64, t471: f64, t473: f64, t6695: f64, t20849: f64, t487: f64, t5812: f64, t602: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21394 = t5219 * t1811;
    let t21439 = t6564 * t1284;
    let t21471 = t3302 * t471;
    let t21541 = t473 * t6695;
    let t21621 = t20849 * t487;
    let t21663 = t5812 * t602;
    (t21394, t21439, t21471, t21541, t21621, t21663)
}
