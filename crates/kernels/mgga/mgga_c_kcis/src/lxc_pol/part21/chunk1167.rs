//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1167/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1167<F: Float>(t26686: F, t3040: F, t4781: F, t14382: F, t3200: F, t95911: F, t2173: F, t46978: F, t8041: F, t7690: F, t96305: F, t14654: F, t3489: F, t27785: F, t2822: F, t14570: F, t2175: F, t26685: F, t7693: F, t93171: F, t93173: F, t93704: F, t93714: F, t95860: F) -> (F, F, F, F) {
    let t96372 = t26686 * t4781 * t3040;
    let t96379 = t3200 * t95911 * t14382;
    let t96382 = t2173 * t46978 * t8041;
    let t96388 = t7690 * t96305;
    let t96391 = t14654 * t3489;
    let t96395 = t2822 * t27785;
    let t96396 = 0.14739506172839506172e-2 * t96395;
    let t96397 = -0.92754700520833333333e-4 * t26685 * t96372 + 0.18550940104166666667e-3 * t26685 * t95860 - 0.3684876543209876543e-3 * t93171 + 0.66327777777777777776e-2 * t96379 - 0.15445601851851851852e-3 * t96382 + 0.37069444444444444444e-2 * t14570 * t3489 * t2175 - 0.46336805555555555556e-3 * t93704 - 0.20612155671296296296e-4 * t96388 - 0.16489724537037037037e-3 * t93714 - 0.4946917361111111111e-3 * t96391 * t7693 - 0.22109259259259259258e-2 * t93173 + t96396;
    (t96372, t96379, t96395, t96397)
}
