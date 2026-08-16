//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1831/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1831<F: Float>(t1307: F, t19577: F, t1530: F, t2379: F, t22960: F, t57893: F, t2745: F, t25373: F, t25: F, t40772: F, t2749: F, t1408: F, t2752: F) -> (F, F, F, F, F, F, F, F, F) {
    let t86685 = t19577 * t1307;
    let t86706 = t1530 * t2379;
    let t86707 = t22960 * t86706;
    let t86710 = t22960 * t57893;
    let t86713 = t1530 * t2745;
    let t86714 = t25373 * t86713;
    let t86716 = t40772 * t25;
    let t86717 = t1530 * t2749;
    let t86718 = t86716 * t86717;
    let t86721 = t2752 * t1408;
    (t86685, t86706, t86707, t86710, t86713, t86714, t86717, t86718, t86721)
}
