//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta501 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1811;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1812;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1813;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1814;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1815;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta501(t3707: f64, t7617: f64, t2134: f64, t3682: f64, t1234: f64, t7623: f64, t1210: f64, t8945: f64, t487: f64, t7642: f64, t11239: f64, t1276: f64, t2148: f64, t2142: f64, t3596: f64, t1269: f64, t3140: f64, t1243: f64, t8939: f64, t2149: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26873, t26877, t26880) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1811(t3707, t7617, t2134, t3682, t1234, t7623);
        let t26889 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1812(t1210, t8945);
        let (t26894, t26895) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1813(t487, t7642, t8945);
        let (t26906, t26907, t26918, t26921) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1814(t11239, t487, t1276, t2148, t2142, t3596, t1269, t3140, t1243, t8939);
        let t26922 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1815(t2149, t26921);
    (t26873, t26877, t26880, t26889, t26894, t26895, t26906, t26907, t26918, t26921, t26922)
}
