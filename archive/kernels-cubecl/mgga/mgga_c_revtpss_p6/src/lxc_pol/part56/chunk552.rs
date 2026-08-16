//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 552/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk552<F: Float>(t4772: F, t996: F, t1678: F, t994: F, t1668: F, t73: F, t3095: F, t3092: F, t3093: F, t357: F, t1592: F, t1058: F, t1660: F) -> (F, F, F, F, F, F) {
    let t4773 = t996 * t4772;
    let t4778 = t994 * t1678;
    let t4781 = t1668 * t73;
    let t4782 = t4781 * t3095;
    let t4783 = t3092 * t4782;
    let t4786 = t3093 * t357;
    let t4787 = t1592 * t4786;
    let t4788 = t3092 * t4787;
    let t4792 = t1660 * t1058;
    (t4773, t4778, t4781, t4783, t4788, t4792)
}
