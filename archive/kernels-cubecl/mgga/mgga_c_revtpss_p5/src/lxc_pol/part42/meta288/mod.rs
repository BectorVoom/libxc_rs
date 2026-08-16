//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta288 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1046;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1047;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta288<F: Float>(t2482: F, t27: F, t2719: F, t820: F, t843: F, t821: F, t235: F, t231: F, t2723: F, t2710: F, t826: F, t9732: F, t234: F, t2735: F, t10631: F, t808: F, t2699: F, t798: F, t802: F, t159: F, t853: F, t216: F, t2729: F, t794: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10850, t10858, t10867, t10868, t10871, t10885) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1046::<F>(t2482, t27, t2719, t820, t843, t821, t235, t231, t2723, t2710, t826, t9732);
        let (t10886, t10888, t10890, t10891, t10900, t10905) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1047::<F>(t234, t2735, t10631, t808, t2699, t798, t802, t159, t853, t216, t2729, t794);
    (t10850, t10858, t10867, t10868, t10871, t10885, t10886, t10888, t10890, t10891, t10900, t10905)
}
