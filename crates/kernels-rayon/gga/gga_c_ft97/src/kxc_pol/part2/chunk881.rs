//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 881/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk881(t13683: f64, t13684: f64, t26: f64, t666: f64, t2360: f64, t743: f64, t1131: f64, t2506: f64, t2355: f64, t13353: f64, t13357: f64, t13362: f64, t13370: f64, t13375: f64, t13379: f64, t13384: f64, t13388: f64, t13391: f64, t13674: f64, t13677: f64, t13680: f64, t13682: f64, t3051: f64, t462: f64, t92: f64, t9903: f64, t9907: f64, t9935: f64, t9958: f64, t9960: f64) -> (f64, f64) {
    let t13685 = t13683 * t13684;
    let t13688 = t26 * t666;
    let t13689 = t743 * t2360;
    let t13690 = t13689 * t13684;
    let t13693 = t2506 * t1131;
    let t13694 = t13693 * t2355;
    let t13697 = 4.0_f64 / 3.0_f64 * t462 * t13353 - 2.0_f64 / 3.0_f64 * t462 * t13357 - 2.0_f64 / 3.0_f64 * t462 * t13362 - 2.0_f64 / 9.0_f64 * t9903 - 8.0_f64 / 27.0_f64 * t9907 + t9958 / 9.0_f64 + 2.0_f64 / 27.0_f64 * t9960 - 6.0_f64 * t462 * t13370 + 4.0_f64 * t462 * t13375 - t9935 + t462 * t13379 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t462 * t13384 - t13388 + 2.0_f64 / 3.0_f64 * t462 * t13391 - t92 * t13674 + 2.0_f64 / 3.0_f64 * t3051 * t13677 - 4.0_f64 / 9.0_f64 * t13680 + 4.0_f64 / 9.0_f64 * t13682 * t13685 - 4.0_f64 / 3.0_f64 * t13688 * t13690 - 4.0_f64 / 3.0_f64 * t13688 * t13694;
    (t13688, t13697)
}
