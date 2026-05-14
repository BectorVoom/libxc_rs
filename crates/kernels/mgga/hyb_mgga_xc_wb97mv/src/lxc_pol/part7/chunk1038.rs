//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1038/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1038<F: Float>(t214: F, t4066: F, t674: F, t1283: F, t1312: F, t191: F, t4109: F, t2025: F, t4126: F, t683: F, t4122: F, t1232: F, t8562: F, t3: F, t3299: F, t2044: F, t3988: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t10588 = t4066 * t214;
    let t10589 = t10588 * t674;
    let t10593 = t1283 * t1312;
    let t10594 = t10593 * t674;
    let t10598 = t191 * t4109;
    let t10599 = t10598 * t674;
    let t10604 = t683 * t2025 * t4126;
    let t10607 = t683 * t2025 * t4122;
    let t10609 = t8562 * t1232;
    let t10613 = t3299 * t3;
    let t10617 = t2044 * t3988;
    (t10588, t10589, t10593, t10594, t10598, t10599, t10604, t10607, t10609, t10613, t10617)
}
