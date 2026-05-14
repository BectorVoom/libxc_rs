//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 606/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk606<F: Float>(t1078: F, t3757: F, t128: F, t2206: F, t1033: F, t311: F, t3297: F, t2580: F, t3679: F, t2578: F, t188: F, t2566: F, t277: F, t334: F, t1084: F, t3687: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3758 = t3757 * t1078;
    let t3760 = t2206 * t128;
    let t3761 = t3760 * t1033;
    let t3763 = t311 * t3761 * t3297;
    let t3765 = t3679 * t2580;
    let t3766 = t2578 * t3765;
    let t3768 = t2566 * t188;
    let t3769 = t277 * t3768;
    let t3770 = t3769 * t334;
    let t3772 = t1084 * t3687;
    (t3758, t3760, t3761, t3763, t3765, t3766, t3768, t3769, t3770, t3772)
}
