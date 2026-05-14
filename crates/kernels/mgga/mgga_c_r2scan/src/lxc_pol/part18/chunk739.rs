//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 739/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk739<F: Float>(t2794: F, t410: F, t2755: F, t1823: F, t963: F, t2747: F, t741: F, t1827: F, t1693: F, t898: F, t2483: F, t697: F, t1721: F, t5393: F, t5: F, t736: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7708 = t410 * t2794;
    let t7720 = 8.0 * t410 * t2755;
    let t7721 = t963 * t1823;
    let t7724 = 0.23392894490538584828e1 * t2747 * t741;
    let t7725 = t963 * t1827;
    let t7727 = t898 * t1693;
    let t7730 = 0.1301229756036208781e0 * t2483 * t697;
    let t7737 = t898 * t1721;
    let t7739 = 48.0 * t5393;
    let t7741 = t2483 * t5;
    let t7743 = 0.10843581300301739842e-1 * t7741 * t736;
    (t7708, t7720, t7721, t7724, t7725, t7727, t7730, t7737, t7739, t7743)
}
