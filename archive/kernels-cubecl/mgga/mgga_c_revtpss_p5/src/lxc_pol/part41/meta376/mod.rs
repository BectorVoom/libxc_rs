//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta376 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1241;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta376<F: Float>(t11354: F, t6113: F, t918: F, t4598: F, t4606: F, t2880: F, t6120: F, t11358: F, t4614: F, t2897: F, t18950: F, t916: F) -> (F, F, F, F, F, F, F) {
        let (t18980, t18982, t18985, t18988, t18990, t18993, t18995) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1241::<F>(t11354, t6113, t918, t4598, t4606, t2880, t6120, t11358, t4614, t2897, t18950, t916);
    (t18980, t18982, t18985, t18988, t18990, t18993, t18995)
}
