//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta636 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2085;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta636<F: Float>(t33: F, t41154: F, t1711: F, t2411: F, t1497: F, t6977: F, t1927: F, t4241: F, t644: F, t7719: F, t13272: F, t607: F) -> (F, F, F, F, F, F) {
        let (t100981, t100987, t101214, t101218, t101226, t101230) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2085::<F>(t33, t41154, t1711, t2411, t1497, t6977, t1927, t4241, t644, t7719, t13272, t607);
    (t100981, t100987, t101214, t101218, t101226, t101230)
}
