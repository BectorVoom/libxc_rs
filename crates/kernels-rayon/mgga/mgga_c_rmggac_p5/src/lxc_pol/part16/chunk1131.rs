//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1131/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1131(t1743: f64, t698: f64, t1756: f64, t118: f64, t326: f64, t333: f64, t352: f64, t44169: f64, t46599: f64, t46603: f64, t46605: f64, t46607: f64, t46609: f64, t48122: f64, t48894: f64, t48897: f64, t49394: f64, t5155: f64, t5266: f64, t8940: f64) -> (f64, f64, f64) {
    let t49407 = t698 * t1743;
    let t49411 = t698 * t1756;
    let t49424 = 0.47896966807455234256e0_f64 * t5155 * t49394 * t333 + t44169 - 0.39914139006212695214e-1_f64 * t118 * t48122 + 0.11974241701863808564e0_f64 * t5266 * t49407 * t352 + 0.11974241701863808564e0_f64 * t8940 * t49411 * t352 + 0.11974241701863808564e0_f64 * t46599 + 0.95793933614910468512e0_f64 * t46603 - 0.5987120850931904282e-1_f64 * t46605 + 0.8980681276397856423e-1_f64 * t46607 - 0.59871208509319042821e-1_f64 * t326 * t48894 - 0.11974241701863808564e0_f64 * t326 * t48897 + 0.5987120850931904282e-1_f64 * t46609;
    (t49407, t49411, t49424)
}
