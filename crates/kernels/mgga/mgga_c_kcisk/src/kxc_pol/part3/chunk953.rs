//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 953/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk953<F: Float>(t14013: F, t14045: F, t14081: F, t14109: F, t1216: F, t1349: F, t13966: F, t13967: F, t13969: F, t13971: F, t13973: F, t13975: F, t13978: F, t13982: F, t1402: F, t338: F, t3729: F, t3904: F, t417: F) -> (F, F) {
    let t14111 = t14013 + t14045 + t14081 + t14109;
    let t14115 = -t13966 - F::cast_from(0.14055920378328537299e-1_f64) * t13967 - F::cast_from(0.28111840756657074597e-1_f64) * t13969 + F::cast_from(0.70279601891642686494e-2_f64) * t13971 + F::cast_from(0.14055920378328537299e-1_f64) * t13973 - F::cast_from(0.23426533963880895498e-2_f64) * t1349 * t13975 - F::cast_from(0.46853067927761790996e-2_f64) * t417 * t13978 - F::cast_from(0.42167761134985611897e-1_f64) * t1349 * t13982 - F::cast_from(3.0_f64) * t3729 * t1402 - t338 * t14111 - F::cast_from(3.0_f64) * t1216 * t3904;
    (t14111, t14115)
}
