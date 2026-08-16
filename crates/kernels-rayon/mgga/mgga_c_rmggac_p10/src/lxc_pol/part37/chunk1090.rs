//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1090/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1090(t1587: f64, t3282: f64, t75876: f64, t75881: f64, t15907: f64, t504: f64, t70048: f64, t70050: f64, t71661: f64, t739: f64, t75853: f64, t75869: f64, t75874: f64, t78322: f64, t78324: f64, t78327: f64, t78339: f64, t78340: f64, t78341: f64, t78349: f64) -> (f64, f64) {
    let t80341 = t3282 * t1587;
    let t80344 = 0.65053455985619242964e-5_f64 * t75876;
    let t80345 = 0.65053455985619242964e-5_f64 * t75881;
    let t80346 = t75853 - t78322 - t78324 + t78327 - 0.57000320883372412499e-7_f64 * t70048 - 0.57000320883372412499e-7_f64 * t70050 + t71661 - t78339 + t78340 + t78341 + 0.76860658247009135562e-5_f64 * t75869 + t75874 - 0.19957069503106347607e-1_f64 * t504 * t15907 - 0.59871208509319042821e-1_f64 * t739 * t80341 + t80344 + t80345 - t78349;
    (t80341, t80346)
}
