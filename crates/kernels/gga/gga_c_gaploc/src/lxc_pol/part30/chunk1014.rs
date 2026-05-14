//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1014/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1014<F: Float>(t29439: F, t9652: F, t2554: F, t7064: F, t7280: F, t21665: F, t9633: F, t9760: F, t21451: F, t5539: F, t9647: F, t21784: F, t7276: F, t3240: F, t7211: F, t2549: F, t9630: F) -> (F, F, F, F, F, F, F, F, F) {
    let t29473 = 0.2563508743380741428e-2 * t29439 * t9652;
    let t29476 = 0.1281754371690370714e-2 * t7064 * t7280 * t2554;
    let t29478 = 0.1281754371690370714e-2 * t21665 * t9633;
    let t29480 = 0.1281754371690370714e-2 * t21665 * t9760;
    let t29483 = 0.2563508743380741428e-2 * t9647 * t5539 * t21451;
    let t29486 = 0.1281754371690370714e-2 * t9647 * t5539 * t21784;
    let t29489 = 0.1281754371690370714e-2 * t7064 * t7276 * t2554;
    let t29492 = 0.64087718584518535698e-3 * t7211 * t3240;
    let t29494 = 0.1281754371690370714e-2 * t2549 * t9630;
    (t29473, t29476, t29478, t29480, t29483, t29486, t29489, t29492, t29494)
}
