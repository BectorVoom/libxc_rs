//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1126/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1126(t22960: f64, t46252: f64, t25373: f64, t46362: f64, t2249: f64, t776: f64, t2553: f64, t606: f64, t25: f64, t9516: f64, t868: f64, t2749: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t81489 = t22960 * t46252;
    let t81492 = t25373 * t46362;
    let t81501 = t2249 * t776;
    let t81505 = t606 * t2553;
    let t81509 = t25 * t9516;
    let t81513 = t2249 * t868;
    let t81521 = t606 * t2749;
    (t81489, t81492, t81501, t81505, t81509, t81513, t81521)
}
