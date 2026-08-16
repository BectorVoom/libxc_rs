//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2436/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2436(t10139: f64, t1398: f64, t281: f64, t543: f64, t624: f64, t1433: f64, t39545: f64, t546: f64, t685: f64, t39552: f64, t557: f64, t1438: f64, t40317: f64) -> (f64, f64, f64, f64) {
    let t46505 = t10139 * t281 * t624 * t1398 * t543;
    let t46515 = 0.65457331274007190912e-5_f64 * t39545 * t546 * t1433 * t685;
    let t46518 = 0.88356352675825229576e-3_f64 * t39552 * t557;
    let t46526 = t40317 * t1438;
    (t46505, t46515, t46518, t46526)
}
