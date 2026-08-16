//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1015/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1015(t78390: f64, t75972: f64, t75978: f64, t15464: f64, t5016: f64, t9128: f64, t70149: f64, t70156: f64, t71717: f64, t71720: f64, t70169: f64, t70173: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t78391 = 0.39914139006212695213e-1_f64 * t78390;
    let t78394 = 0.1702583995731913576e-4_f64 * t75972;
    let t78395 = 0.11634323970834742769e-4_f64 * t75978;
    let t78396 = t5016 * t15464;
    let t78397 = 0.2993560425465952141e-1_f64 * t78396;
    let t78398 = t9128 * t15464;
    let t78399 = 0.2993560425465952141e-1_f64 * t78398;
    let t78400 = 0.54549323308490683456e-1_f64 * t70149;
    let t78401 = 0.21819729323396273382e0_f64 * t70156;
    let t78402 = 0.40650199722100037752e-3_f64 * t71717;
    let t78403 = 0.40650199722100037752e-3_f64 * t71720;
    let t78404 = 0.72042316457491791901e-3_f64 * t70169;
    let t78405 = 0.38430329123504567781e-4_f64 * t70173;
    (t78391, t78394, t78395, t78397, t78399, t78400, t78401, t78402, t78403, t78404, t78405)
}
