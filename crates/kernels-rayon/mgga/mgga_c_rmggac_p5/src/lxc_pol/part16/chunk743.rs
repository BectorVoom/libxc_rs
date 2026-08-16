//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 743/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk743(t131: f64, t1341: f64, t2019: f64, t4789: f64, t640: f64, t649: f64, t49: f64, t288: f64, t290: f64, t2010: f64, t2139: f64, t27: f64, t3118: f64, t333: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34790 = t131 * t1341;
    let t34793 = t2019 * t649 * t4789 * t640 * t34790;
    let t34795 = t4789 * t49;
    let t34796 = t34795 * t288;
    let t34797 = t290 * t34790;
    let t34799 = t2010 * t34796 * t34797;
    let t34803 = t2139 * t27 * t3118 * t333;
    (t34790, t34793, t34795, t34797, t34799, t34803)
}
