//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta517 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1819;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1820;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta517<F: Float>(t138: F, t785: F, t9302: F, t2452: F, t9720: F, t675: F, t886: F, t11006: F, t256: F, t10115: F, t251: F, t2410: F, t10308: F, t599: F, t90: F, t29: F, t560: F, t9655: F, t1389: F, t268: F, t555: F, t4146: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t40270, t40688, t41040, t41077, t41117, t41153) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1819::<F>(t138, t785, t9302, t2452, t9720, t675, t886, t11006, t256, t10115, t251, t2410);
        let (t41154, t45963, t45972, t46361, t46808, t47567, t47671) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1820::<F>(t41153, t10308, t599, t90, t29, t560, t9655, t1389, t268, t10115, t555, t4146);
    (t40270, t40688, t41040, t41077, t41117, t41154, t45963, t45972, t46361, t46808, t47567, t47671)
}
