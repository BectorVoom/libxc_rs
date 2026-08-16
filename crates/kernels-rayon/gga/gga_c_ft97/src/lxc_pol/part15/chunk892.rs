//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 892/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk892(t2248: f64, t419: f64, t934: f64, t3139: f64, t959: f64, t2999: f64, t89: f64, t943: f64, t3000: f64, t921: f64, t8417: f64, t971: f64) -> (f64, f64, f64, f64, f64) {
    let t45662 = t419 * t2248 * t934;
    let t46019 = t3139 * t959;
    let t46256 = t89 * t2999 * t943;
    let t46320 = t89 * t3000 * t921;
    let t46565 = t971 * t8417;
    (t45662, t46019, t46256, t46320, t46565)
}
