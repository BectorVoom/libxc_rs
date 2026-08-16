//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 195/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk195(t1179: f64, t205: f64, t209: f64, t28: f64, t484: f64, t465: f64, t479: f64, t31: f64, t198: f64, t673: f64, t476: f64, t77: f64, t9: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1180 = t205 * t1179;
    let t1184 = t209 * t209;
    let t1189 = t484 * t28;
    let t1190 = t465 * t1189;
    let t1191 = t1190 * t479;
    let t1193 = t465 * t31;
    let t1194 = t673 * t198;
    let t1195 = t1193 * t1194;
    let t1196 = t476 * t209;
    let t1223 = 1.0_f64 / t9 / t77;
    (t1180, t1184, t1189, t1190, t1191, t1193, t1195, t1196, t1223)
}
