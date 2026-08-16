//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta528 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1942;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1943;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta528<F: Float>(t2014: F, t28182: F, t25190: F, t7900: F, t5542: F, t7312: F, t7315: F, t7934: F, t7235: F, t7901: F, t7937: F, t2013: F, t8995: F, t2033: F, t9593: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t28183, t28184, t28186, t28187, t28188, t28189, t28190, t28192, t28193, t28196) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1942::<F>(t2014, t28182, t25190, t7900, t5542, t7312, t7315, t7934, t7235, t7901, t7937, t2013, t8995);
        let t28197 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1943::<F>(t2033, t9593);
    (t28183, t28184, t28186, t28187, t28188, t28189, t28190, t28192, t28193, t28196, t28197)
}
