//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 267/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk267(t2106: f64, t655: f64, t2069: f64, t851: f64, t2074: f64, t854: f64, t265: f64, t344: f64, t22: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2107 = t655 * t2106;
    let t2108 = 0.30305179615828157477e-2_f64 * t2107;
    let t2109 = t851 * t2069;
    let t2111 = t854 * t2074;
    let t2113 = t344 * t265;
    let t2114 = 0.17701538806747441785e-3_f64 * t2113;
    let t2115 = t854 * t22;
    (t2107, t2108, t2109, t2111, t2113, t2114, t2115)
}
