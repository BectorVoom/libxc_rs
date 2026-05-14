//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1094/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1094<F: Float>(t12: F, t9720: F, t9728: F, t8729: F, t1151: F, t1153: F, t3000: F, t3005: F, t318: F, t319: F, t3706: F, t3710: F, t808: F, t810: F, t201: F, t199: F, t3719: F, t967: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t84 = t12 <= zeta_threshold;
    let t203 = rho0 <= dens_threshold || t84;
    let t9729 = t9720 + t9728;
    let t9738 = piecewise3(t84, 0.0, t8729);
    let t9742 = piecewise3(t203, 0.0, t9729 * t319 / 2.0 + t3706 * t810 / 2.0 + t3000 * t1153 + t1151 * t3005 + t808 * t3710 / 2.0 + t318 * t9738 / 2.0);
    let t9743 = t201 * t9742;
    let t9744 = t199 * t9743;
    let t9746 = t3719 * t967;
    (t9729, t9738, t9743, t9744, t9746)
}
