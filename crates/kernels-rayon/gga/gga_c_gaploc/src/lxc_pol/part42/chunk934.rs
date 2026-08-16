//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 934/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk934(t11318: f64, t2464: f64, t2465: f64, t587: f64, t2365: f64, t36211: f64, t7025: f64, t10430: f64, t9263: f64, t993: f64, t11718: f64, t7324: f64) -> (f64, f64, f64, f64) {
    let t46815 = t587 * t2464 * t2465 * t11318;
    let t46818 = t7025 * t2365 * t36211;
    let t46819 = 0.14896037479937677779e-1_f64 * t46818;
    let t46821 = t9263 * t993 * t10430;
    let t46832 = 2.0_f64 * t7324 * t11718;
    (t46815, t46819, t46821, t46832)
}
