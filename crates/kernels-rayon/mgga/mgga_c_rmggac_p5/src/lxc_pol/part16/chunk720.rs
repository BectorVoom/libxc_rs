//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 720/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk720(t10492: f64, t10420: f64, t884: f64, t10085: f64, t10091: f64, t10096: f64, t10098: f64, t10103: f64, t10107: f64, t1916: f64, t708: f64, t10113: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10493 = 0.11974241701863808564e0_f64 * t10492;
    let t10494 = t884 * t10420;
    let t10495 = 0.59871208509319042821e-1_f64 * t10494;
    let t10497 = 0.5107751987195740728e-4_f64 * t10085;
    let t10498 = 0.5107751987195740728e-4_f64 * t10091;
    let t10499 = 0.1702583995731913576e-4_f64 * t10096;
    let t10500 = 0.1702583995731913576e-4_f64 * t10098;
    let t10501 = 0.638468998399467591e-4_f64 * t10103;
    let t10502 = 0.15323255961587222184e-3_f64 * t10107;
    let t10505 = t1916 * t708;
    let t10506 = 0.19957069503106347607e-1_f64 * t10505;
    let t10507 = 0.5987120850931904282e-1_f64 * t10113;
    (t10493, t10495, t10497, t10498, t10499, t10500, t10501, t10502, t10506, t10507)
}
