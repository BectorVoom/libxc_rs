//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta565 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1888;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1889;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta565<F: Float>(t2453: F, t27212: F, t1032: F, t4469: F, t867: F, t786: F, t1955: F, t7063: F, t1568: F, t25410: F, t25374: F, t98848: F, t33: F, t41154: F, t1711: F, t2411: F, t1497: F, t6977: F, t1927: F, t4241: F, t644: F, t7719: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t99257, t99272, t99303, t99373, t99403, t99404, t99463) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1888::<F>(t2453, t27212, t1032, t4469, t867, t786, t1955, t7063, t1568, t25410, t25374, t98848);
        let (t99466, t100981, t100987, t101214, t101218, t101226) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1889::<F>(t25374, t99403, t33, t41154, t1711, t2411, t1497, t6977, t1927, t4241, t644, t7719);
    (t99257, t99272, t99303, t99373, t99404, t99463, t99466, t100981, t100987, t101214, t101218, t101226)
}
