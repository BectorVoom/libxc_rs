//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta968 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3266;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3267;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3268;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta968<F: Float>(t30: F, t48294: F, t1317: F, t22790: F, t1320: F, t13550: F, t13553: F, t18280: F, t21906: F, t2255: F, t22670: F, t22769: F, t3833: F, t47025: F, t513: F, t5549: F, t605: F, t76396: F, t85406: F, t85409: F, zeta_threshold: F, t33: F, t1113: F, t13565: F, t13568: F, t20256: F, t21918: F, t22778: F, t22783: F, t3841: F, t47040: F, t516: F, t5557: F, t81123: F, t85426: F, t85429: F, t162: F, t187: F, t48297: F, t48304: F, t48306: F, t47093: F, t39989: F, t47084: F, t47086: F, t47088: F, t47092: F, t47096: F, t47098: F, t48300: F, t48303: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t85928, t85930, t85932, t85950) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3266::<F>(t30, t48294, t1317, t22790, t1320, t13550, t13553, t18280, t21906, t2255, t22670, t22769, t3833, t47025, t513, t5549, t605, t76396, t85406, t85409, zeta_threshold);
        let t85968 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3267::<F>(t33, t1113, t13565, t13568, t20256, t21918, t2255, t22778, t22783, t3841, t47040, t516, t5557, t81123, t85426, t85429, zeta_threshold);
        let (t85970, t85972, t85973, t85974, t85975, t85976, t85977) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3268::<F>(t162, t85950, t85968, t187, t48297, t48304, t48306, t47093, t39989, t47084, t47086, t47088, t47092, t47096, t47098, t48300, t48303, t85928, t85930, t85932);
    (t85928, t85930, t85932, t85970, t85972, t85973, t85974, t85975, t85976, t85977)
}
