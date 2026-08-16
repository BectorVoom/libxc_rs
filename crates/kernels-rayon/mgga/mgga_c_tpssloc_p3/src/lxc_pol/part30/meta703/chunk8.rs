//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2293/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2293(t1933: f64, t23479: f64, t99665: f64, t1015: f64, t23472: f64, t28586: f64, t17615: f64, t6717: f64, t17620: f64, t23422: f64, t28558: f64, t28566: f64, t5890: f64, t5894: f64, t5909: f64, t6723: f64, t83008: f64, t88648: f64, t88689: f64, t88692: f64) -> f64 {
    let t99774 = t1933 * t99665 * t23479;
    let t99779 = t23472 * t1015 * t28586;
    let t99785 = t6717 * t17615;
    let t99789 = t6717 * t17620;
    let t99793 = t88648 - 0.10093189023535097714e-3_f64 * t99774 + 0.80745512188280781712e-3_f64 * t6723 * t28566 - t88689 - t88692 + 0.10093189023535097714e-3_f64 * t99779 + t83008 * t5909 / 1152.0_f64 - t23422 * t5890 / 108.0_f64 + t99785 / 864.0_f64 - t23422 * t5894 / 81.0_f64 + t99789 / 648.0_f64 + 0.80745512188280781712e-3_f64 * t6723 * t28558;
    t99793
}
