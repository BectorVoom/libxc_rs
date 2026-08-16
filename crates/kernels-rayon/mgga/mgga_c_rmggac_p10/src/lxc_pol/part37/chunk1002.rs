//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1002/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1002(t2471: f64, t265: f64, t305: f64, t76373: f64, t76375: f64, t76377: f64, t76379: f64, t76381: f64, t69213: f64, t69234: f64, t69241: f64, t69250: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t78112 = t2471 * t265;
    let t78113 = t305 * t78112;
    let t78114 = 0.39914139006212695213e-1_f64 * t78113;
    let t78115 = 0.20455996240684006298e-1_f64 * t76373;
    let t78116 = 0.20455996240684006298e-1_f64 * t76375;
    let t78117 = 0.2727466165424534173e-1_f64 * t76377;
    let t78119 = 0.2727466165424534173e-1_f64 * t76379;
    let t78120 = 0.54549323308490683461e-1_f64 * t76381;
    let t78122 = 0.77145928569998943516e-3_f64 * t69213;
    let t78123 = 0.16566831523319392755e-1_f64 * t69234;
    let t78124 = 0.27611385872198987926e-1_f64 * t69241;
    let t78125 = 0.72732431077987577944e-1_f64 * t69250;
    (t78112, t78114, t78115, t78116, t78117, t78119, t78120, t78122, t78123, t78124, t78125)
}
