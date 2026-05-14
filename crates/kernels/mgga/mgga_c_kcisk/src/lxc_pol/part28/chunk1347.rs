//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1347/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1347<F: Float>(t113203: F, t24091: F, t11701: F, t7444: F, t9967: F, t24495: F, t2799: F, t5218: F, t24098: F, t33071: F, t11694: F, t35280: F, t63008: F, t112011: F, t8968: F, t9094: F, t9718: F) -> (F, F, F, F, F, F, F, F) {
    let t120987 = 6.0 * t113203 * t24091;
    let t120990 = 12.0 * t11701 * t9967 * t7444;
    let t120993 = 2.0 * t5218 * t2799 * t24495;
    let t120995 = 2.0 * t33071 * t24098;
    let t120997 = 2.0 * t11694 * t35280;
    let t120999 = 4.0 * t63008 * t9967;
    let t121001 = 2.0 * t112011 * t8968;
    let t121004 = 2.0 * t5218 * t9718 * t9094;
    (t120987, t120990, t120993, t120995, t120997, t120999, t121001, t121004)
}
