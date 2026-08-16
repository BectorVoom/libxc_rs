//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 245/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk245(t147: f64, t362: f64, t135: f64, t376: f64, t377: f64) -> (f64, f64, f64, f64, f64) {
    let t1088 = t362 * t147;
    let t1089 = 1.0_f64 / t1088;
    let t1090 = t135 * t1089;
    let t1091 = t376 * t376;
    let t1092 = t1091 * t377;
    let t1094 = 2.0_f64 * t1090 * t1092;
    (t1089, t1090, t1091, t1092, t1094)
}
