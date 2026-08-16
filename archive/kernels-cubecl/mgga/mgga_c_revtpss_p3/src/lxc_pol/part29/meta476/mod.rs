//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta476 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1751;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1752;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta476<F: Float>(t4292: F, t94: F, t1353: F, t1907: F, t30: F, t892: F, t4433: F, t18875: F, t25207: F, t1544: F, t605: F, t4343: F, t1032: F, t1568: F, t1955: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t27126, t27153, t27159, t27160, t27166, t27169, t27173) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1751::<F>(t4292, t94, t1353, t1907, t30, t892, t4433, t18875, t25207, t1544, t605, t4343);
        let (t27198, t27199) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1752::<F>(t1032, t1568, t1955);
    (t27126, t27153, t27159, t27160, t27166, t27169, t27173, t27198, t27199)
}
