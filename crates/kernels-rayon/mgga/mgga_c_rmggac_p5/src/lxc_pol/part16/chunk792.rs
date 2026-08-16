//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 792/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk792(t2004: f64, t38638: f64, t2007: f64, t1987: f64, t1990: f64, t1652: f64, t7778: f64, t739: f64, t7364: f64, t8576: f64, t16156: f64, t8508: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t38639 = t38638 * t2004;
    let t38643 = t38638 * t2007;
    let t38645 = t38638 * t1987;
    let t38647 = t38638 * t1990;
    let t38674 = t7778 * t1652;
    let t38675 = t739 * t38674;
    let t38701 = t8576 * t7364;
    let t38704 = t16156 * t8508;
    (t38639, t38643, t38645, t38647, t38674, t38675, t38701, t38704)
}
