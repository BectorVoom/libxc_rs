//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta585 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1818;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1819;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1820;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta585<F: Float>(t30: F, t48292: F, t48294: F, t85929: F, t85931: F, t21906: F, t22670: F, t3833: F, t47025: F, t513: F, t5549: F, t5824: F, t87125: F, t91797: F, t91802: F, zeta_threshold: F, t33: F, t21918: F, t22783: F, t3841: F, t47040: F, t516: F, t5557: F, t6416: F, t89780: F, t91811: F, t91816: F, t162: F, t189: F, t512: F, t48297: F, t48304: F, t48306: F, t39989: F, t47084: F, t47086: F, t47088: F, t47092: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t91982, t91983, t91984, t91985, t91997) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1818::<F>(t30, t48292, t48294, t85929, t85931, t21906, t22670, t3833, t47025, t513, t5549, t5824, t87125, t91797, t91802, zeta_threshold);
        let t92011 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1819::<F>(t33, t21918, t22783, t3841, t47040, t516, t5557, t6416, t89780, t91811, t91816, t162, t91997, zeta_threshold);
        let (t92013, t92014, t92015, t92016, t92017) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1820::<F>(t189, t512, t92011, t48297, t48304, t48306, t39989, t47084, t47086, t47088, t47092, t91982, t91983, t91984, t91985);
    (t91982, t91983, t91984, t91985, t92011, t92013, t92014, t92015, t92016, t92017)
}
