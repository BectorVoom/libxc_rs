//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 227/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk227(t131: f64, t934: f64, t290: f64, t356: f64, t274: f64, t49: f64, t288: f64) -> (f64, f64, f64, f64) {
    let t935 = t934 * t131;
    let t938 = t290 * t356;
    let t941 = t274 * t49;
    let t942 = t941 * t288;
    (t935, t938, t941, t942)
}
