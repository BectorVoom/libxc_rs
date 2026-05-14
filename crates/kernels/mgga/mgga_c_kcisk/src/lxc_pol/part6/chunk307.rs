//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 307/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk307<F: Float>(t662: F, t695: F, t661: F, t657: F, t667: F, t1333: F, t721: F, t690: F) -> (F, F, F, F, F, F, F, F) {
    let t1776 = t662 * t695;
    let t1781 = t661 * t661;
    let t1782 = 1.0 / t1781;
    let t1783 = t657 * t1782;
    let t1791 = 1.0 / t667;
    let t1795 = t1333 * t721;
    let t1796 = 0.16581944444444444444e-2 * t1795;
    let t1797 = 1.0 / t690;
    (t1776, t1781, t1782, t1783, t1791, t1795, t1796, t1797)
}
