//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 738/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk738<F: Float>(t1831: F, t963: F, t2747: F, t750: F, t1842: F, t1814: F, t5249: F, t897: F, t5252: F, t2743: F, t5326: F, t1419: F, t959: F, t2483: F, t725: F, t41: F) -> (F, F, F, F, F, F, F, F) {
    let t7685 = t963 * t1831;
    let t7688 = 0.34631718211362927518e2 * t2747 * t750;
    let t7689 = t963 * t1842;
    let t7691 = t963 * t1814;
    let t7693 = t5249 * t897;
    let t7694 = t7693 * t5252;
    let t7699 = t2743 * t5326;
    let t7701 = t1419 * t959;
    let t7705 = t2483 * t725;
    let t7707 = 2.0 * t41 * t7705;
    (t7685, t7688, t7689, t7691, t7694, t7699, t7701, t7707)
}
