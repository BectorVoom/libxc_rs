//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 714/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk714(t13534: f64, t13566: f64, t11701: f64, t977: f64, t3459: f64, t8862: f64, t3638: f64, t7324: f64, t5559: f64, t1052: f64, t3511: f64, t2592: f64, t3684: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13567 = t13534 + t13566;
    let t13569 = t11701 * t977;
    let t13573 = 4.0_f64 * t8862 * t3459;
    let t13577 = 2.0_f64 * t7324 * t3638;
    let t13578 = t3638 * t977;
    let t13580 = 6.0_f64 * t5559 * t13578;
    let t13581 = t1052 * t3511;
    let t13584 = t2592 * t3684;
    (t13567, t13569, t13573, t13577, t13578, t13580, t13581, t13584)
}
