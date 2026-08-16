//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 827/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk827(t2754: f64, t587: f64, t9438: f64, t9439: f64, t40007: f64, t40009: f64, t40013: f64, t40015: f64, t40019: f64, t40021: f64, t40023: f64, t2877: f64, t40394: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t41711 = t587 * t9438 * t9439 * t2754;
    let t41712 = 0.15976219147466979032e-1_f64 * t41711;
    let t41713 = 0.29792074959875355558e-1_f64 * t40007;
    let t41714 = 0.63904876589867916127e-1_f64 * t40009;
    let t41715 = 0.63904876589867916127e-1_f64 * t40013;
    let t41716 = 0.63904876589867916127e-1_f64 * t40015;
    let t41717 = 0.63904876589867916127e-1_f64 * t40019;
    let t41718 = 0.29792074959875355558e-1_f64 * t40021;
    let t41719 = 0.29792074959875355558e-1_f64 * t40023;
    let t41721 = 0.35750489951850426669e0_f64 * t40394 * t2877;
    (t41712, t41713, t41714, t41715, t41716, t41717, t41718, t41719, t41721)
}
