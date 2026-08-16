//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta224 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1425;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1426;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1427;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1428;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1429;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1430;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1431;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1432;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta224(t3302: f64, t3603: f64, t1248: f64, t5332: f64, t1269: f64, t1287: f64, t1794: f64, t487: f64, t5284: f64, t3781: f64, t460: f64, t471: f64, t1811: f64, t473: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t5464 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1425(t3302, t3603);
        let t5465 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1426(t1248, t5464);
        let t5466 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1427(t5332, t5465);
        let (t5470, t5474, t5477) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1428(t1269, t1287, t1794, t487, t5284, t3781);
        let t5478 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1429(t460, t5477);
        let t5480 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1430(t1248, t3302, t471);
        let t5481 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1431(t5332, t5480);
        let t5486 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1432(t1811, t473);
    (t5464, t5465, t5466, t5470, t5474, t5477, t5478, t5480, t5481, t5486)
}
