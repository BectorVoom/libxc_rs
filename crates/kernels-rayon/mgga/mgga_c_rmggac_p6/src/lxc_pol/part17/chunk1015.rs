//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1015/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1015(t1743: f64, t2064: f64, t797: f64, t2367: f64, t558: f64, t1652: f64, t27055: f64, t333: f64, t352: f64, t41116: f64, t46622: f64, t46669: f64, t46671: f64, t46673: f64, t46675: f64, t46677: f64, t46679: f64, t5155: f64, t5266: f64, t838: f64, t8936: f64, t8940: f64) -> (f64, f64, f64) {
    let t46685 = t2064 * t1743;
    let t46686 = t797 * t46685;
    let t46694 = t2367 * t558;
    let t46701 = -0.5987120850931904282e-1_f64 * t46669 - 0.17961362552795712846e0_f64 * t46671 - 0.17961362552795712846e0_f64 * t46673 + 0.8980681276397856423e-1_f64 * t46675 + 0.35922725105591425692e0_f64 * t46677 + 0.23948483403727617128e0_f64 * t838 * t46679 - 0.35922725105591425692e0_f64 * t27055 * t46622 * t333 + 0.11974241701863808564e0_f64 * t46686 + 0.23948483403727617128e0_f64 * t8940 * t8936 * t1652 - 0.47896966807455234256e0_f64 * t41116 * t46622 * t352 + 0.23948483403727617128e0_f64 * t5266 * t46694 * t352 + 0.47896966807455234256e0_f64 * t5155 * t46694 * t333;
    (t46685, t46694, t46701)
}
