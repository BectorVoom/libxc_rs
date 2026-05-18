//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1031/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1031<F: Float>(t1354: F, t30941: F, t1349: F, t13966: F, t20796: F, t20798: F, t25557: F, t25559: F, t25561: F, t25563: F, t25925: F, t30900: F, t30909: F, t417: F) -> F {
    let t30942 = t1354 * t30941;
    let t30946 = -F::new(0.42167761134985611897e-1) * t1349 * t30900 - t13966 - F::new(0.14055920378328537299e-1) * t20796 - F::new(0.28111840756657074597e-1) * t20798 + F::new(0.14055920378328537299e-1) * t25557 - F::new(0.14055920378328537299e-1) * t25559 + F::new(0.70279601891642686494e-2) * t25561 - F::new(0.42167761134985611897e-1) * t25563 - F::new(0.23426533963880895498e-2) * t1349 * t30909 - F::new(0.46853067927761790996e-2) * t417 * t30942 - F::new(0.28111840756657074597e-1) * t25925;
    t30946
}
