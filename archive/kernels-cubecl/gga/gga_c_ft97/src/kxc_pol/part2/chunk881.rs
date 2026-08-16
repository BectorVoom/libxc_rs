//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 881/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk881<F: Float>(t13683: F, t13684: F, t26: F, t666: F, t2360: F, t743: F, t1131: F, t2506: F, t2355: F, t13353: F, t13357: F, t13362: F, t13370: F, t13375: F, t13379: F, t13384: F, t13388: F, t13391: F, t13674: F, t13677: F, t13680: F, t13682: F, t3051: F, t462: F, t92: F, t9903: F, t9907: F, t9935: F, t9958: F, t9960: F) -> (F, F) {
    let t13685 = t13683 * t13684;
    let t13688 = t26 * t666;
    let t13689 = t743 * t2360;
    let t13690 = t13689 * t13684;
    let t13693 = t2506 * t1131;
    let t13694 = t13693 * t2355;
    let t13697 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t462 * t13353 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t462 * t13357 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t462 * t13362 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t9903 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t9907 + t9958 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t9960 - F::cast_from(6.0_f64) * t462 * t13370 + F::cast_from(4.0_f64) * t462 * t13375 - t9935 + t462 * t13379 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t462 * t13384 - t13388 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t462 * t13391 - t92 * t13674 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t3051 * t13677 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t13680 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t13682 * t13685 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t13688 * t13690 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t13688 * t13694;
    (t13688, t13697)
}
