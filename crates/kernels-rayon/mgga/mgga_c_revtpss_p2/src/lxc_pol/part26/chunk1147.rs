//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1147/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1147(t25986: f64, t2661: f64, t9769: f64, t25978: f64, t4014: f64, t25972: f64, t9923: f64, t2453: f64, t4086: f64, t64: f64, t9795: f64, t2018: f64, t40688: f64, t46808: f64) -> (f64, f64, f64, f64, f64) {
    let t94557 = t2661 * t25986 * t9769;
    let t94559 = t25978 * t4014;
    let t94561 = t25972 * t9923;
    let t94564 = t2453 * t4086 * t64;
    let t94565 = t94564 * t9795;
    let t94568 = t40688 * t2018 * t46808;
    (t94557, t94559, t94561, t94565, t94568)
}
