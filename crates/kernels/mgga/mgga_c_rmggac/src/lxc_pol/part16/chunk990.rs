//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 990/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk990<F: Float>(t551: F, t9565: F, t2447: F, t1614: F, t1652: F, t27048: F, t305: F, t321: F, t333: F, t352: F, t41116: F, t4669: F, t46710: F, t46715: F, t46737: F, t49184: F, t49394: F, t49480: F, t49493: F, t5148: F, t5259: F, t5266: F, t838: F, t8940: F, t9523: F, t9551: F) -> (F, F, F) {
    let t49557 = t9565 * t551;
    let t49560 = t2447 * t551;
    let t49567 = -0.35922725105591425692e0 * t4669 * t9523 * t1614 - 0.47896966807455234256e0 * t41116 * t49480 * t352 - 0.11974241701863808564e0 * t46710 + 0.23948483403727617128e0 * t5266 * t49493 * t333 - 0.23948483403727617128e0 * t5148 * t49493 * t321 - 0.11974241701863808564e0 * t46715 + 0.23948483403727617128e0 * t838 * t49184 + 0.23948483403727617128e0 * t5266 * t49394 * t352 + 0.35922725105591425692e0 * t27048 * t49480 * t321 - 0.2993560425465952141e-1 * t46737 + 0.11974241701863808564e0 * t305 * t49557 + 0.23948483403727617128e0 * t5259 * t49560 * t321 + 0.23948483403727617128e0 * t8940 * t9551 * t1652;
    (t49557, t49560, t49567)
}
