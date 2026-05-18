//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 730/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk730<F: Float>(t20731: F, t2179: F, t574: F, t167: F, t20027: F, t2205: F, t12617: F, t16969: F, t1901: F, t20685: F, t20690: F, t20694: F, t20698: F, t20702: F, t20706: F, t20711: F, t20716: F, t20720: F, t20725: F, t20729: F, t446: F) -> (F, F, F) {
    let t20733 = t574 * t2179 * t20731;
    let t20737 = t2205 * t167 * t20027;
    let t20741 = t1901 * t20685 / F::new(3.0) - F::new(4.0) / F::new(27.0) * t12617 - t446 * t20690 / F::new(3.0) - F::new(2.0) / F::new(9.0) * t446 * t20694 - t446 * t20698 / F::new(9.0) - F::new(10.0) / F::new(81.0) * t446 * t20702 - t446 * t20706 / F::new(3.0) - F::new(2.0) * t446 * t20711 + F::new(2.0) * t446 * t20716 + F::new(2.0) * t446 * t20720 + t446 * t20725 + t446 * t20729 - F::new(2.0) * t446 * t20733 + F::new(4.0) / F::new(9.0) * t446 * t20737 - F::new(2.0) / F::new(3.0) * t16969;
    (t20733, t20737, t20741)
}
