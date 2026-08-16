//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 775/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk775(t2084: f64, t798: f64, t7603: f64, t7599: f64, t25525: f64, t27: f64, t35917: f64, t3851: f64, t3826: f64, t35884: f64, t3814: f64, t35871: f64, t793: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t36114 = t2084 * t798;
    let t36115 = t7603 * t36114;
    let t36117 = t7599 * t36114;
    let t36119 = t25525 * t27;
    let t36127 = t3851 * t35917;
    let t36141 = t3826 * t35917;
    let t36152 = t3814 * t35884;
    let t36154 = t793 * t35871;
    (t36115, t36117, t36119, t36127, t36141, t36152, t36154)
}
