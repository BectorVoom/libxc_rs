//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1051/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1051(t76379: f64, t76381: f64, t69213: f64, t69234: f64, t69241: f64, t69250: f64, t69261: f64, t75300: f64, t75304: f64, t69274: f64, t75308: f64, t69201: f64, t69206: f64, t69245: f64, t71269: f64, t71270: f64, t71271: f64, t71272: f64) -> (f64, f64, f64) {
    let t78119 = 0.2727466165424534173e-1_f64 * t76379;
    let t78120 = 0.54549323308490683461e-1_f64 * t76381;
    let t78122 = 0.77145928569998943516e-3_f64 * t69213;
    let t78123 = 0.16566831523319392755e-1_f64 * t69234;
    let t78124 = 0.27611385872198987926e-1_f64 * t69241;
    let t78125 = 0.72732431077987577944e-1_f64 * t69250;
    let t78126 = 0.10286123809333192469e-2_f64 * t69261;
    let t78127 = 0.77145928569998943515e-3_f64 * t75300;
    let t78128 = 0.10286123809333192468e-2_f64 * t75304;
    let t78129 = 0.26609426004141796809e-1_f64 * t69274;
    let t78130 = 0.19914231157590872009e-2_f64 * t75308;
    let t78131 = t69201 - 0.33868944250243438615e-2_f64 * t69206 - t78122 - t78123 + t78124 + t69245 - t78125 + t78126 + t71269 + t71270 - t71271 + t71272 - t78127 + t78128 - t78129 + t78130;
    (t78119, t78120, t78131)
}
