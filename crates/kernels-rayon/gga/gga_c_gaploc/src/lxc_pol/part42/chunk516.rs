//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 516/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk516(t7403: f64, t959: f64, t7340: f64, t3281: f64, t5676: f64, t2530: f64, t2610: f64, t2365: f64, t2033: f64, t1645: f64, t2586: f64, t3307: f64, t9420: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9935 = 0.29792074959875355558e-1_f64 * t7403 * t959;
    let t9937 = 0.29792074959875355558e-1_f64 * t7340 * t959;
    let t9942 = 0.29792074959875355558e-1_f64 * t5676 * t3281;
    let t9943 = t2610 * t2530;
    let t9944 = t2365 * t9943;
    let t9946 = 0.29792074959875355558e-1_f64 * t2033 * t9944;
    let t9972 = t1645 * t2586;
    let t9981 = t9420 * t3307;
    (t9935, t9937, t9942, t9946, t9972, t9981)
}
