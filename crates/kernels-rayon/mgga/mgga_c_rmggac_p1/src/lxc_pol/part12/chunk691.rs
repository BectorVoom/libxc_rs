//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 691/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk691(t2292: f64, t4965: f64, t7204: f64, t8902: f64, t7192: f64, t8906: f64, t5888: f64, t875: f64, t1971: f64, t3351: f64, t2310: f64, t7720: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9131 = t4965 * t2292;
    let t9133 = t7204 * t8902;
    let t9135 = t7192 * t8906;
    let t9137 = t875 * t5888;
    let t9138 = t1971 * t9137;
    let t9139 = t3351 * t9138;
    let t9143 = t7720 * t2310;
    (t9131, t9133, t9135, t9138, t9139, t9143)
}
