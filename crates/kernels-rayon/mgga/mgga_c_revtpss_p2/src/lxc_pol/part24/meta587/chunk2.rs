//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1826/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1826(t46800: f64, t46810: f64, t46817: f64, t46820: f64, t46824: f64, t46831: f64, t46840: f64, t48792: f64, t74429: f64, t74437: f64, t85873: f64, t85885: f64, t86061: f64, t86070: f64, t86074: f64, t86078: f64, t86080: f64) -> f64 {
    let t92136 = t46800 + t46810 - t46817 + t46820 - t46824 + 7.0_f64 / 36.0_f64 * t85873 - 0.17149607247227894789e-3_f64 * t85885 - 0.50820002809285328224e-4_f64 * t86061 + 0.17149607247227894789e-2_f64 * t86070 - 0.30492001685571196935e-3_f64 * t86074 + 0.30492001685571196935e-3_f64 * t86078 + 0.40015750243531754508e-2_f64 * t86080 - t46831 + t46840 + 0.81312004494856525159e-3_f64 * t74429 - 0.51384669507166276316e-2_f64 * t48792 - 0.1084295579938911763e-3_f64 * t74437;
    t92136
}
