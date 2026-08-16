//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta331 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1251;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1252;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta331<F: Float>(t12865: F, t3717: F, t1263: F, t675: F, t1122: F, t247: F, t1261: F, t126: F, t3617: F, t1231: F, t3655: F, t2434: F, t371: F, t482: F, t481: F, t11262: F, t1251: F, t1247: F, t1284: F, t3566: F, t3624: F, t828: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t12866, t12879, t12882, t12884, t12893, t12898) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1251::<F>(t12865, t3717, t1263, t675, t1122, t247, t1261, t126, t3617, t1231, t3655, t2434, t371, t482);
        let (t12900, t12905, t12910, t12915, t12916) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1252::<F>(t12898, t481, t11262, t1251, t1247, t1284, t3566, t3624, t126, t482, t828);
    (t12866, t12879, t12882, t12884, t12893, t12900, t12905, t12910, t12915, t12916)
}
