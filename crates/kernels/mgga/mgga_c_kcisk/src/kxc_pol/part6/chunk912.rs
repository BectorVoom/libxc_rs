//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 912/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk912<F: Float>(t13244: F, t1355: F, t19182: F, t2083: F, t25623: F, t306: F, t30605: F, t30616: F, t30916: F, t30938: F, t3599: F, t5687: F, t7757: F, t7764: F, t1354: F, t1349: F, t13966: F, t20796: F, t20798: F, t25557: F, t25559: F, t25561: F, t25563: F, t25925: F, t30900: F, t30909: F, t417: F) -> (F, F) {
    let t30941 = 3.0 / 16.0 * t13244 * t30616 - 3.0 / 8.0 * t19182 * t7757 - 3.0 / 8.0 * t3599 * t30916 + 3.0 / 4.0 * t25623 * t2083 + 3.0 / 4.0 * t5687 * t7764 + t1355 * t30605 / 4.0 + t306 * t30938 / 2.0;
    let t30942 = t1354 * t30941;
    let t30946 = -0.42167761134985611897e-1 * t1349 * t30900 - t13966 - 0.14055920378328537299e-1 * t20796 - 0.28111840756657074597e-1 * t20798 + 0.14055920378328537299e-1 * t25557 - 0.14055920378328537299e-1 * t25559 + 0.70279601891642686494e-2 * t25561 - 0.42167761134985611897e-1 * t25563 - 0.23426533963880895498e-2 * t1349 * t30909 - 0.46853067927761790996e-2 * t417 * t30942 - 0.28111840756657074597e-1 * t25925;
    (t30941, t30946)
}
