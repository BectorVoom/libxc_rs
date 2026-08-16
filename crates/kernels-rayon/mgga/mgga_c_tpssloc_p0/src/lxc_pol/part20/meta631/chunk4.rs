//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2300/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2300(t12895: f64, t12971: f64, t193: f64, t202: f64, t2522: f64, t2553: f64, t262: f64, t4314: f64, t46481: f64, t47149: f64, t47151: f64, t47153: f64, t47156: f64, t47159: f64, t47161: f64, t47162: f64, t47164: f64, t47564: f64, t47593: f64, t47631: f64, t776: f64, t870: f64) -> f64 {
    let t47644 = t193 * t202 * (t46481 + t47564 + t47593 + t47631) * t870 + t47149 + t47151 + t47153 + 9.0_f64 * t2522 * t12895 * t2553 + t47156 + t47159 + t47161 + t47162 + 18.0_f64 * t4314 * t262 * t12971 * t776 + t47164;
    t47644
}
