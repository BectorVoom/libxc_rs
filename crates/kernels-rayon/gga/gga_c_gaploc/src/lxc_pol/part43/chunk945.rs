//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 945/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk945(t46849: f64, t6508: f64, t1358: f64, t6507: f64, t2321: f64, t38674: f64, t9074: f64, t1365: f64, t38281: f64, t38277: f64, t4261: f64, t13749: f64, t203: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46850 = t6508 * t46849;
    let t46852 = t1358 * t6507 * t46850;
    let t46859 = t9074 * t38674 * t2321;
    let t46862 = t9074 * t1365 * t38281;
    let t46865 = t9074 * t4261 * t38277;
    let t46867 = t203 * t13749;
    (t46850, t46852, t46859, t46862, t46865, t46867)
}
