//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta968 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3266;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3267;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3268;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta968(t30: f64, t48294: f64, t1317: f64, t22790: f64, t1320: f64, t13550: f64, t13553: f64, t18280: f64, t21906: f64, t2255: f64, t22670: f64, t22769: f64, t3833: f64, t47025: f64, t513: f64, t5549: f64, t605: f64, t76396: f64, t85406: f64, t85409: f64, zeta_threshold: f64, t33: f64, t1113: f64, t13565: f64, t13568: f64, t20256: f64, t21918: f64, t22778: f64, t22783: f64, t3841: f64, t47040: f64, t516: f64, t5557: f64, t81123: f64, t85426: f64, t85429: f64, t162: f64, t187: f64, t48297: f64, t48304: f64, t48306: f64, t47093: f64, t39989: f64, t47084: f64, t47086: f64, t47088: f64, t47092: f64, t47096: f64, t47098: f64, t48300: f64, t48303: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t85928, t85930, t85932, t85950) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3266(t30, t48294, t1317, t22790, t1320, t13550, t13553, t18280, t21906, t2255, t22670, t22769, t3833, t47025, t513, t5549, t605, t76396, t85406, t85409, zeta_threshold);
        let t85968 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3267(t33, t1113, t13565, t13568, t20256, t21918, t2255, t22778, t22783, t3841, t47040, t516, t5557, t81123, t85426, t85429, zeta_threshold);
        let (t85970, t85972, t85973, t85974, t85975, t85976, t85977) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3268(t162, t85950, t85968, t187, t48297, t48304, t48306, t47093, t39989, t47084, t47086, t47088, t47092, t47096, t47098, t48300, t48303, t85928, t85930, t85932);
    (t85928, t85930, t85932, t85970, t85972, t85973, t85974, t85975, t85976, t85977)
}
