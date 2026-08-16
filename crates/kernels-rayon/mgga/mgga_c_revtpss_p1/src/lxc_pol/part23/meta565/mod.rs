//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta565 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2138;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2139;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2140;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2141;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2142;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta565(t30: f64, t13611: f64, t1468: f64, t6785: f64, t22670: f64, t513: f64, t5549: f64, t5824: f64, t9335: f64, t1711: f64, t6792: f64, zeta_threshold: f64, t33: f64, t516: f64, t5557: f64, t6416: f64, t9350: f64, t162: f64, t189: f64, t512: f64, t1344: f64, t5574: f64, t9605: f64, t1348: f64, t5582: f64, t9617: f64, t1868: f64, t6836: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22768, t22769, t22777, t22778, t22783) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2138(t30, t13611, t1468, t6785, t22670, t513, t5549, t5824, t9335, t1711, t6792, zeta_threshold);
        let t22789 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2139(t33, t22778, t22783, t516, t5557, t6416, t9350, t162, t22777, zeta_threshold);
        let (t22790, t22791, t22799, t22807) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2140(t30, t33, t189, t22789, t512, t1344, t22670, t22769, t5574, t5824, t9605, t1348, t22778, t22783, t5582, t6416, t9617, zeta_threshold);
        let t22809 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2141(t22799, t22807);
        let t22813 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2142(t1868, t6836);
    (t22768, t22769, t22778, t22783, t22789, t22790, t22791, t22809, t22813)
}
