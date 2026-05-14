//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 742/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk742<F: Float>(t10845: F, t4430: F, t1558: F, t853: F, t4353: F, t9794: F, t10760: F, t10890: F, t1549: F, t4416: F, t808: F, t10886: F, t2710: F, t2713: F, t4371: F, t10744: F) -> (F, F, F, F, F, F, F, F) {
    let t14716 = t10845 * t4430;
    let t14718 = t853 * t1558;
    let t14760 = t9794 * t4353;
    let t14761 = t10760 * t14760;
    let t14765 = t10890 * t1549;
    let t14779 = t808 * t4416;
    let t14780 = t10886 * t14779;
    let t14817 = t2710 * t2713 * t4371;
    let t14819 = t808 * t4353;
    let t14820 = t10744 * t14819;
    (t14716, t14718, t14760, t14761, t14765, t14780, t14817, t14820)
}
