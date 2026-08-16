//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1385/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1385(t1429: f64, t39501: f64, t544: f64, t9989: f64, t555: f64, t4003: f64, t1433: f64, t39545: f64, t546: f64, t685: f64, t39552: f64, t557: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46412 = 0.56911289235245161963e-1_f64 * t39501 * t1429;
    let t46475 = 1.0_f64 / t9989 / t544;
    let t46476 = t46475 * t555;
    let t46478 = t4003 * t4003;
    let t46515 = 0.65457331274007190912e-5_f64 * t39545 * t546 * t1433 * t685;
    let t46518 = 0.88356352675825229576e-3_f64 * t39552 * t557;
    (t46412, t46475, t46476, t46478, t46515, t46518)
}
