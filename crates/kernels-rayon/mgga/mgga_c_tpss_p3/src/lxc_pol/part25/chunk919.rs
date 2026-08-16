//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 919/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk919(t1193: f64, t8110: f64, t2222: f64, t3190: f64, t1186: f64, t3211: f64, t27: f64, t558: f64, t498: f64, t1190: f64, t8124: f64, t1173: f64, t3280: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9956 = 0.35089341735807877242e1_f64 * t1193 * t8110;
    let t9957 = t3190 * t2222;
    let t9959 = t3211 * t1186;
    let t9965 = t558 * t27;
    let t9966 = t9965 * t498;
    let t9972 = 0.56968947174242584612e-3_f64 * t1190 * t8124;
    let t9980 = 12.0_f64 * t1173 * t3280;
    (t9956, t9957, t9959, t9966, t9972, t9980)
}
