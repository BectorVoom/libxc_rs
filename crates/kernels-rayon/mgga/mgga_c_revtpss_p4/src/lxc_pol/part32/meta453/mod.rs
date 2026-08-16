//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta453 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1648;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1649;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1650;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta453(t25227: f64, t2664: f64, t2661: f64, t2670: f64, t7033: f64, t2482: f64, t27: f64, t7043: f64, t2677: f64, t1941: f64, t243: f64, t2712: f64, t64: f64, t2710: f64, t826: f64, t7036: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25228, t25230, t25231, t25234) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1648(t25227, t2664, t2661, t2670, t7033, t2482, t27, t7043);
        let (t25236, t25237, t25240) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1649(t25234, t2677, t1941, t243, t2712, t64);
        let (t25242, t25245) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1650(t25240, t2710, t826, t2482, t27, t7036);
    (t25228, t25230, t25231, t25234, t25236, t25237, t25240, t25242, t25245)
}
