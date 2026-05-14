//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1214/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1214<F: Float>(t1184: F, t6199: F, t1208: F, t2318: F, t2239: F, t3030: F, t1171: F, t6198: F, t8040: F, t881: F, t2317: F, t3113: F, t1201: F, t6230: F, t8227: F, t862: F) -> (F, F, F, F, F, F, F, F) {
    let t22684 = t6199 * t1184;
    let t22699 = t2318 * t1208;
    let t22722 = t3030 * t2239;
    let t22727 = t1171 * t6198;
    let t22740 = t8040 * t881;
    let t22745 = t3113 * t2317;
    let t22750 = t1201 * t6230;
    let t22757 = t8227 * t862;
    (t22684, t22699, t22722, t22727, t22740, t22745, t22750, t22757)
}
