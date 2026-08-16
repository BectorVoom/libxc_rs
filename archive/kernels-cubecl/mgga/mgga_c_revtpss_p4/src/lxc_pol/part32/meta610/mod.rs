//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta610 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1949;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta610<F: Float>(t1711: F, t4343: F, t106561: F, t27799: F, t105923: F, t25759: F, t11064: F, t27384: F, t106533: F, t100987: F, t18875: F, t4433: F, t892: F) -> (F, F, F, F, F, F, F) {
        let (t107901, t107908, t107919, t107924, t107927, t107930, t107934) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1949::<F>(t1711, t4343, t106561, t27799, t105923, t25759, t11064, t27384, t106533, t100987, t18875, t4433, t892);
    (t107901, t107908, t107919, t107924, t107927, t107930, t107934)
}
