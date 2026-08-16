//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1067/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1067(t2228: f64, t2350: f64, t903: f64, t15467: f64, t4601: f64, t1550: f64, t699: f64, t8704: f64, t75859: f64, t75864: f64, t75866: f64, t1356: f64, t14434: f64, t235: f64, t5144: f64, t515: f64, t5267: f64, t5888: f64, t70048: f64, t70050: f64, t71158: f64, t71661: f64, t739: f64, t75869: f64, t75874: f64, t75876: f64, t75881: f64, t78184: f64, t884: f64) -> f64 {
    let t78321 = t903 * t2228 * t2350;
    let t78322 = 0.44903406381989282115e-1_f64 * t78321;
    let t78323 = t4601 * t15467;
    let t78324 = 0.44903406381989282115e-1_f64 * t78323;
    let t78326 = t1550 * t699 * t8704;
    let t78327 = 0.2993560425465952141e-1_f64 * t78326;
    let t78339 = 0.44903406381989282115e-1_f64 * t75859;
    let t78340 = 0.38430329123504567781e-4_f64 * t75864;
    let t78341 = 0.38430329123504567781e-4_f64 * t75866;
    let t78348 = -t78322 - t78324 + t78327 + 0.11974241701863808564e0_f64 * t739 * t14434 * t5144 - 0.11974241701863808564e0_f64 * t884 * t14434 * t5267 - 0.11974241701863808564e0_f64 * t1356 * t71158 * t5888 - 0.57000320883372412496e-7_f64 * t70048 - 0.57000320883372412496e-7_f64 * t70050 + t71661 - t78339 + t78340 + t78341 + 0.76860658247009135557e-5_f64 * t75869 - 0.19957069503106347607e-1_f64 * t235 * t515 * t78184 + t75874 + 0.6505345598561924296e-5_f64 * t75876 + 0.6505345598561924296e-5_f64 * t75881;
    t78348
}
