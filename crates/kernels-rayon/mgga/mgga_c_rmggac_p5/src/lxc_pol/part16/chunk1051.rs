//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1051/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1051(t46420: f64, t7204: f64, t46424: f64, t7192: f64, t46428: f64, t8620: f64, t46431: f64, t8640: f64, t10100: f64, t236: f64, t498: f64, t7230: f64, t7248: f64) -> (f64, f64, f64, f64, f64) {
    let t47885 = t7204 * t46420;
    let t47887 = t7192 * t46424;
    let t47889 = t8620 * t46428;
    let t47891 = t8640 * t46431;
    let t47898 = t7230 * t7248 * t236 * t10100 * t498;
    (t47885, t47887, t47889, t47891, t47898)
}
