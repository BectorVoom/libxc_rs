//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1058/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1058(t78200: f64, t72023: f64, t8902: f64, t72020: f64, t8906: f64, t22: f64, t326: f64, t8041: f64, t8622: f64, t118: f64, t338: f64, t76414: f64, t77488: f64, t77525: f64, t77720: f64, t78119: f64, t78120: f64, t78184: f64, t78189: f64, t78194: f64, t78199: f64) -> f64 {
    let t78201 = 0.27274661654245341728e-1_f64 * t78200;
    let t78202 = t72023 * t8902;
    let t78203 = 0.20455996240684006297e-1_f64 * t78202;
    let t78204 = t72020 * t8906;
    let t78205 = 0.27274661654245341729e-1_f64 * t78204;
    let t78207 = t326 * t8041 * t22;
    let t78208 = t78207 * t8622;
    let t78209 = 0.20455996240684006297e-1_f64 * t78208;
    let t78210 = -t78119 - t78120 + 0.19957069503106347607e-1_f64 * t118 * t338 * t78184 - t78189 - 0.59871208509319042821e-1_f64 * t326 * t77720 - 0.59871208509319042821e-1_f64 * t326 * t77525 - t78194 - 0.39914139006212695214e-1_f64 * t118 * t77488 + t78199 + t78201 - t78203 + t78205 + t78209 - t76414;
    t78210
}
