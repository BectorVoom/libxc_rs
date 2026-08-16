//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1840/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1840(t1888: f64, t25045: f64, t82159: f64, t6562: f64, t7488: f64, t82133: f64, t25225: f64, t6547: f64, t23168: f64, t25338: f64, t23012: f64, t7485: f64) -> (f64, f64, f64, f64, f64) {
    let t86933 = t1888 * t82159 * t25045;
    let t86940 = t6562 * t82133 * t7488;
    let t86942 = t6547 * t25225;
    let t86950 = t23168 * t25338;
    let t86955 = t23012 * t7485;
    (t86933, t86940, t86942, t86950, t86955)
}
