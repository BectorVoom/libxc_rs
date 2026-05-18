//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1090/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1090<F: Float>(t41579: F, t41581: F, t4928: F, t699: F, t41585: F, t41604: F, t10792: F, t1249: F, t2448: F, t2463: F, t27326: F, t40780: F, t41577: F, t41587: F, t41591: F, t41596: F, t41600: F, t41607: F, t41610: F, t5048: F, t5223: F, t8041: F, t884: F) -> (F, F) {
    let t43745 = F::new(0.1489760996265424379e-3) * t41579;
    let t43746 = F::new(0.39726959900411316772e-4) * t41581;
    let t43749 = t699 * t4928;
    let t43752 = F::new(0.11918087970123395032e-3) * t41585;
    let t43757 = F::new(0.60975299583150056624e-3) * t41604;
    let t43760 = -F::new(0.5107751987195740728e-4) * t40780 + F::new(0.11974241701863808564e1) * t5048 * t699 * t5223 + F::new(0.35922725105591425692e0) * t884 * t8041 * t27326 + F::new(0.1702583995731913576e-4) * t41577 - F::new(0.19957069503106347607e-1) * t1249 * t2448 + t43745 + t43746 - F::new(0.59871208509319042821e-1) * t10792 * t2463 + F::new(0.59871208509319042821e-1) * t884 * t43749 - t43752 + F::new(0.10215503974391481456e-3) * t41587 + F::new(0.30646511923174444368e-3) * t41591 + F::new(0.85129199786595678799e-5) * t41596 - F::new(0.2553875993597870364e-4) * t41600 - t43757 - F::new(0.1440846329149835838e-2) * t41607 - F::new(0.1440846329149835838e-2) * t41610;
    (t43749, t43760)
}
