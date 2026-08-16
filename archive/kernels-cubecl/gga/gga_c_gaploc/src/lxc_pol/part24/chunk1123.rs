//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1123/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1123<F: Float>(t21665: F, t9760: F, t21451: F, t5539: F, t9647: F, t21784: F, t2554: F, t7064: F, t7276: F, t3240: F, t7211: F, t2549: F, t9630: F) -> (F, F, F, F, F, F) {
    let t29480 = F::cast_from(0.1281754371690370714e-2_f64) * t21665 * t9760;
    let t29483 = F::cast_from(0.2563508743380741428e-2_f64) * t9647 * t5539 * t21451;
    let t29486 = F::cast_from(0.1281754371690370714e-2_f64) * t9647 * t5539 * t21784;
    let t29489 = F::cast_from(0.1281754371690370714e-2_f64) * t7064 * t7276 * t2554;
    let t29492 = F::cast_from(0.64087718584518535698e-3_f64) * t7211 * t3240;
    let t29494 = F::cast_from(0.1281754371690370714e-2_f64) * t2549 * t9630;
    (t29480, t29483, t29486, t29489, t29492, t29494)
}
