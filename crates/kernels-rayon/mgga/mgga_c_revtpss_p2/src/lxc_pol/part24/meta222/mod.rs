//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta222 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk973;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk974;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk975;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta222(t12166: f64, t342: f64, t11631: f64, t12051: f64, t1129: f64, t3431: f64, t408: f64, t3434: f64, t421: f64, t418: f64, t240: f64, t3698: f64, t3361: f64, t635: f64, t57: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12167, t12168, t12226, t12227, t12230, t12247, t12248, t12254) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk973(t12166, t342, t11631, t12051, t1129, t3431, t408, t3434, t421, t418, t240, t3698);
        let t12256 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk974(t3361, t635);
        let (t12267, t12268) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk975(t3361, t57);
    (t12167, t12168, t12226, t12227, t12230, t12247, t12248, t12254, t12256, t12267, t12268)
}
