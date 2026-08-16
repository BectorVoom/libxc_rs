//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta209 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1235;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1236;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1237;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1238;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta209(t45: f64, t57: f64, t4397: f64, t2375: f64, t5819: f64, t5825: f64, t78: f64, t2382: f64, t81: f64, t162: f64, t187: f64, t150: f64, t190: f64, t1522: f64, t4311: f64, zeta_threshold: f64, t4399: f64, t766: f64, t80: f64, t770: f64, t83: f64, t1544: f64, t4546: f64, t1558: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5927, t5940, t5941, t5943, t5944, t5945, t5947) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1235(t45, t57, t4397, t2375, t5819, t5825, t78, t2382, t81, t162, t187, t150, t190, t1522, t4311, zeta_threshold);
        let (t5948, t5962) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1236(t45, t57, t4399, t5819, t5825, t766, t80, t770, t83, zeta_threshold);
        let t5966 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1237(t1544);
        let (t5970, t5977) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1238(t1544, t4546, t1558);
    (t5927, t5940, t5941, t5943, t5944, t5945, t5947, t5948, t5962, t5966, t5970, t5977)
}
