//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta502 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1819;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta502(t2452: f64, t9720: f64, t675: f64, t886: f64, t11006: f64, t256: f64, t10115: f64, t251: f64, t2410: f64, t2240: f64, t2246: f64, t10308: f64, t599: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t40688, t41040, t41077, t41117, t41154, t45958, t45963) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1819(t2452, t9720, t675, t886, t11006, t256, t10115, t251, t2410, t2240, t2246, t10308, t599);
    (t40688, t41040, t41077, t41117, t41154, t45958, t45963)
}
