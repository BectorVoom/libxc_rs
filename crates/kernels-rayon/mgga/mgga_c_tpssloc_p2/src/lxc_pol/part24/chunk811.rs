//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 811/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk811(t590: f64, t60: f64, t192: f64, t533: f64, t1390: f64, t2018: f64, t584: f64, t16: f64, t2: f64, t591: f64, t9: f64, t21: f64, t587: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8705 = 1.0_f64 / t60 / t590;
    let t8944 = t192 * t533;
    let t8945 = t2018 * t1390;
    let t9211 = 0.1044e2_f64 * t584;
    let t9212 = t2 * t16;
    let t9213 = 0.4332e2_f64 * t9212;
    let t9214 = t9 * t591;
    let t9215 = 0.9288e2_f64 * t9214;
    let t9216 = t587 * t21;
    (t8705, t8944, t8945, t9211, t9212, t9213, t9214, t9215, t9216)
}
