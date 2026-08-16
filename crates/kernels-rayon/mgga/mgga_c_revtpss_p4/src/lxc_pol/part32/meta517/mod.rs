//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta517 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1819;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1820;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta517(t138: f64, t785: f64, t9302: f64, t2452: f64, t9720: f64, t675: f64, t886: f64, t11006: f64, t256: f64, t10115: f64, t251: f64, t2410: f64, t10308: f64, t599: f64, t90: f64, t29: f64, t560: f64, t9655: f64, t1389: f64, t268: f64, t555: f64, t4146: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40270, t40688, t41040, t41077, t41117, t41153) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1819(t138, t785, t9302, t2452, t9720, t675, t886, t11006, t256, t10115, t251, t2410);
        let (t41154, t45963, t45972, t46361, t46808, t47567, t47671) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1820(t41153, t10308, t599, t90, t29, t560, t9655, t1389, t268, t10115, t555, t4146);
    (t40270, t40688, t41040, t41077, t41117, t41154, t45963, t45972, t46361, t46808, t47567, t47671)
}
