//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2634/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2634<F: Float>(t231: F, t2782: F, t2783: F, t6041: F, t836: F, t61756: F, t2797: F, t136: F, t2457: F, t2710: F, t10535: F, t5978: F) -> (F, F, F, F) {
    let t62693 = t2782 * t2783 * t6041 * t836 * t231;
    let t62695 = t61756 * t231;
    let t62697 = t2782 * t2797 * t62695;
    let t62716 = t2710 * t6041 * t136 * t2457;
    let t62723 = t10535 * t5978 * t136 * t2457;
    (t62693, t62697, t62716, t62723)
}
