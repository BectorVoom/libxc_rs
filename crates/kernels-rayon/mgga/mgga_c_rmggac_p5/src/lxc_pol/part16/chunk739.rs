//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 739/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk739(t1295: f64, t1302: f64, t131: f64, t20: f64, t2018: f64, t2020: f64, t252: f64, t640: f64, t7335: f64, t7766: f64, t7334: f64, t7552: f64) -> (f64, f64, f64) {
    let t34704 = t1295 * t1302 * t20 * t2018 * t2020 * t640 * t131 * t252;
    let t34706 = t7335 * t7766;
    let t34709 = t7334 * t7552;
    (t34704, t34706, t34709)
}
