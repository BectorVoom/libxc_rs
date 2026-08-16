//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 706/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk706(t9518: f64, t9546: f64, t9574: f64, t9593: f64, t82: f64, t72: f64, t1685: f64, t702: f64, t2211: f64, t5144: f64, t739: f64, t5267: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9595 = t9518 + t9546 + t9574 + t9593;
    let t9596 = t82 * t9595;
    let t9597 = t72 * t9596;
    let t9598 = t1685 * t702;
    let t9599 = t72 * t9598;
    let t9620 = t2211 * t5144;
    let t9621 = t739 * t9620;
    let t9624 = t2211 * t5267;
    (t9595, t9596, t9597, t9598, t9599, t9620, t9621, t9624)
}
