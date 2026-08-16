//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 868/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk868<F: Float>(t3191: F, t4999: F, t1092: F, t1768: F, t9539: F, t3178: F, t4773: F, t2811: F, t4977: F, t1008: F, t2822: F, t5006: F) -> (F, F, F, F, F, F) {
    let t13366 = t4999 * t3191;
    let t13367 = t1092 * t13366;
    let t13369 = t9539 * t1768;
    let t13370 = t1092 * t13369;
    let t13372 = t3178 * t4773;
    let t13373 = t1092 * t13372;
    let t13376 = t4977 * t2811;
    let t13377 = t13376 * t1008;
    let t13382 = t2822 * t5006;
    (t13367, t13370, t13373, t13376, t13377, t13382)
}
