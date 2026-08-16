//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1090/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1090(t41579: f64, t41581: f64, t4928: f64, t699: f64, t41585: f64, t41604: f64, t10792: f64, t1249: f64, t2448: f64, t2463: f64, t27326: f64, t40780: f64, t41577: f64, t41587: f64, t41591: f64, t41596: f64, t41600: f64, t41607: f64, t41610: f64, t5048: f64, t5223: f64, t8041: f64, t884: f64) -> (f64, f64) {
    let t43745 = 0.1489760996265424379e-3_f64 * t41579;
    let t43746 = 0.39726959900411316772e-4_f64 * t41581;
    let t43749 = t699 * t4928;
    let t43752 = 0.11918087970123395032e-3_f64 * t41585;
    let t43757 = 0.60975299583150056624e-3_f64 * t41604;
    let t43760 = -0.5107751987195740728e-4_f64 * t40780 + 0.11974241701863808564e1_f64 * t5048 * t699 * t5223 + 0.35922725105591425692e0_f64 * t884 * t8041 * t27326 + 0.1702583995731913576e-4_f64 * t41577 - 0.19957069503106347607e-1_f64 * t1249 * t2448 + t43745 + t43746 - 0.59871208509319042821e-1_f64 * t10792 * t2463 + 0.59871208509319042821e-1_f64 * t884 * t43749 - t43752 + 0.10215503974391481456e-3_f64 * t41587 + 0.30646511923174444368e-3_f64 * t41591 + 0.85129199786595678799e-5_f64 * t41596 - 0.2553875993597870364e-4_f64 * t41600 - t43757 - 0.1440846329149835838e-2_f64 * t41607 - 0.1440846329149835838e-2_f64 * t41610;
    (t43749, t43760)
}
