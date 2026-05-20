//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta290 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1522;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta290<F: Float>(t1046: F, t11262: F, t1041: F, t3140: F, t989: F, t3149: F, t3160: F, t2923: F, t910: F, t287: F, t2922: F, t275: F) -> (F, F, F, F, F, F, F) {
        let (t11263, t11264, t11274, t11277, t11294, t11298, t11299) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1522::<F>(t1046, t11262, t1041, t3140, t989, t3149, t3160, t2923, t910, t287, t2922, t275);
    (t11263, t11264, t11274, t11277, t11294, t11298, t11299)
}
