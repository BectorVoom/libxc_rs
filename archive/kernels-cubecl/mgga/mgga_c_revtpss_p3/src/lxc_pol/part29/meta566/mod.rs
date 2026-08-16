//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta566 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1912;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1913;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta566<F: Float>(t4292: F, t648: F, t1907: F, t4144: F, t3829: F, t13514: F, t94: F, t4135: F, t13716: F, t1450: F, t28166: F, t7234: F, t8995: F, t14468: F, t30: F, t2: F, t2411: F, t580: F, t890: F, t892: F, t775: F, t1583: F, t2430: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t98487, t98496, t98519, t98535, t98550, t98564, t98579) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1912::<F>(t4292, t648, t1907, t4144, t3829, t13514, t94, t4135, t13716, t1450, t28166, t7234);
        let (t98588, t98627, t98633, t98648, t98651) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1913::<F>(t7234, t8995, t14468, t30, t2, t2411, t580, t890, t892, t775, t1583, t2430);
    (t98487, t98496, t98519, t98535, t98550, t98564, t98579, t98588, t98627, t98633, t98648, t98651)
}
