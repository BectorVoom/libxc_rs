//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 864/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk864<F: Float>(t1041: F, t15731: F, t1663: F, t371: F, t676: F, t1025: F, t1647: F, t3140: F, t3149: F, t1660: F, t3201: F, t1086: F, t4746: F) -> (F, F, F, F, F, F, F) {
    let t15732 = t1041 * t15731;
    let t15749 = t371 * t676 * t1663;
    let t15750 = t1025 * t15749;
    let t15822 = t1647 * t3140;
    let t15823 = t15822 * t3149;
    let t15862 = t1660 * t3201;
    let t15925 = t4746 * t1086;
    (t15732, t15749, t15750, t15822, t15823, t15862, t15925)
}
