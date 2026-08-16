//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta585 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1818;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1819;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1820;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta585(t30: f64, t48292: f64, t48294: f64, t85929: f64, t85931: f64, t21906: f64, t22670: f64, t3833: f64, t47025: f64, t513: f64, t5549: f64, t5824: f64, t87125: f64, t91797: f64, t91802: f64, zeta_threshold: f64, t33: f64, t21918: f64, t22783: f64, t3841: f64, t47040: f64, t516: f64, t5557: f64, t6416: f64, t89780: f64, t91811: f64, t91816: f64, t162: f64, t189: f64, t512: f64, t48297: f64, t48304: f64, t48306: f64, t39989: f64, t47084: f64, t47086: f64, t47088: f64, t47092: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t91982, t91983, t91984, t91985, t91997) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1818(t30, t48292, t48294, t85929, t85931, t21906, t22670, t3833, t47025, t513, t5549, t5824, t87125, t91797, t91802, zeta_threshold);
        let t92011 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1819(t33, t21918, t22783, t3841, t47040, t516, t5557, t6416, t89780, t91811, t91816, t162, t91997, zeta_threshold);
        let (t92013, t92014, t92015, t92016, t92017) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1820(t189, t512, t92011, t48297, t48304, t48306, t39989, t47084, t47086, t47088, t47092, t91982, t91983, t91984, t91985);
    (t91982, t91983, t91984, t91985, t92011, t92013, t92014, t92015, t92016, t92017)
}
