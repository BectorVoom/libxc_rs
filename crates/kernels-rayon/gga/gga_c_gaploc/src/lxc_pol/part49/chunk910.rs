//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 910/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk910(t12939: f64, t1407: f64, t2754: f64, t587: f64, t9438: f64, t9439: f64, t40007: f64, t40021: f64, t40023: f64, t2877: f64, t40394: f64, t2299: f64, t3338: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t41705 = t1407 * t12939;
    let t41706 = 0.15976219147466979032e-1_f64 * t41705;
    let t41711 = t587 * t9438 * t9439 * t2754;
    let t41712 = 0.15976219147466979032e-1_f64 * t41711;
    let t41713 = 0.29792074959875355558e-1_f64 * t40007;
    let t41718 = 0.29792074959875355558e-1_f64 * t40021;
    let t41719 = 0.29792074959875355558e-1_f64 * t40023;
    let t41721 = 0.35750489951850426669e0_f64 * t40394 * t2877;
    let t41722 = t2299 * t3338;
    (t41706, t41712, t41713, t41718, t41719, t41721, t41722)
}
