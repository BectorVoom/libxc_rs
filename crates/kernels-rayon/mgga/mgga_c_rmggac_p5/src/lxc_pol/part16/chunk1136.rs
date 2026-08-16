//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1136/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1136(t551: f64, t9565: f64, t2447: f64, t1614: f64, t1652: f64, t27048: f64, t305: f64, t321: f64, t333: f64, t352: f64, t41116: f64, t4669: f64, t46710: f64, t46715: f64, t46737: f64, t49184: f64, t49394: f64, t49480: f64, t49493: f64, t5148: f64, t5259: f64, t5266: f64, t838: f64, t8940: f64, t9523: f64, t9551: f64) -> (f64, f64, f64) {
    let t49557 = t9565 * t551;
    let t49560 = t2447 * t551;
    let t49567 = -0.35922725105591425692e0_f64 * t4669 * t9523 * t1614 - 0.47896966807455234256e0_f64 * t41116 * t49480 * t352 - 0.11974241701863808564e0_f64 * t46710 + 0.23948483403727617128e0_f64 * t5266 * t49493 * t333 - 0.23948483403727617128e0_f64 * t5148 * t49493 * t321 - 0.11974241701863808564e0_f64 * t46715 + 0.23948483403727617128e0_f64 * t838 * t49184 + 0.23948483403727617128e0_f64 * t5266 * t49394 * t352 + 0.35922725105591425692e0_f64 * t27048 * t49480 * t321 - 0.2993560425465952141e-1_f64 * t46737 + 0.11974241701863808564e0_f64 * t305 * t49557 + 0.23948483403727617128e0_f64 * t5259 * t49560 * t321 + 0.23948483403727617128e0_f64 * t8940 * t9551 * t1652;
    (t49557, t49560, t49567)
}
