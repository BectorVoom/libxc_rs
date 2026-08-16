//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1326/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1326(t10623: f64, t2952: f64, t10632: f64, t41825: f64, t41827: f64, t959: f64, t10605: f64, t2940: f64, t41977: f64, t942: f64, t951: f64, t41992: f64, t41998: f64, t42002: f64, t42005: f64, t42025: f64, t42031: f64, t42097: f64, t42105: f64) -> (f64, f64, f64, f64, f64) {
    let t42682 = 0.10389515463408878255e3_f64 * t10623 * t2952;
    let t42686 = 0.12304822629859687989e5_f64 * t959 * t41825 * t41827 * t10632;
    let t42688 = 0.23392894490538584828e1_f64 * t2940 * t10605;
    let t42692 = 0.5848223622634646207e0_f64 * t959 * t942 * t41977 * t951;
    let t42693 = t41992 - t41998 - t42002 + t42005 - t42682 + t42025 - t42031 + t42097 + t42105 + t42686 - t42688 - t42692;
    (t42682, t42686, t42688, t42692, t42693)
}
