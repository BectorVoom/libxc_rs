//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta274 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1023;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1024;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta274<F: Float>(t1432: F, t1433: F, t9288: F, t136: F, t1419: F, t2457: F, t3964: F, t225: F, t9646: F, t1428: F, t22: F, t2452: F, t557: F, t1429: F, t9292: F, t4096: F, t9285: F, t1398: F, t215: F, t268: F, t543: F, t4101: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t10102, t10109, t10111, t10114, t10115) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1023::<F>(t1432, t1433, t9288, t136, t1419, t2457, t3964, t225, t9646, t1428, t22, t2452);
        let (t10117, t10126, t10129, t10137) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1024::<F>(t10115, t557, t1429, t9292, t3964, t4096, t9285, t1398, t215, t268, t543, t4101);
    (t10102, t10109, t10111, t10114, t10115, t10117, t10126, t10129, t10137)
}
