//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta537 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1981;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1982;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta537<F: Float>(t2014: F, t28187: F, t7315: F, t7934: F, t7235: F, t7901: F, t7937: F, t2013: F, t8995: F, t2033: F, t9593: F) -> (F, F, F, F, F, F, F) {
        let (t28188, t28189, t28190, t28192, t28193, t28196) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1981::<F>(t2014, t28187, t7315, t7934, t7235, t7901, t7937, t2013, t8995);
        let t28197 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1982::<F>(t2033, t9593);
    (t28188, t28189, t28190, t28192, t28193, t28196, t28197)
}
