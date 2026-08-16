//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta221 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1321;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1322;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1323;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1324;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1325;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta221(t3767: f64, t5330: f64, t1248: f64, t3603: f64, t5332: f64, t3720: f64, t1774: f64, t1250: f64, t1794: f64, t73: f64, t1214: f64, t471: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t5340 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1321(t3767, t5330);
        let t5341 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1322(t1248, t3603);
        let (t5342, t5343, t5346) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1323(t5332, t5341, t3720, t1248, t1774);
        let (t5347, t5348, t5351) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1324(t1250, t5346, t3720, t1794, t73);
        let t5352 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1325(t1214, t471);
    (t5340, t5341, t5342, t5343, t5346, t5347, t5348, t5351, t5352)
}
