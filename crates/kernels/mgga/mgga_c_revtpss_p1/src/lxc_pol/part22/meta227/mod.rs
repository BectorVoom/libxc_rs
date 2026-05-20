//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta227 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1443;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1444;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1445;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1446;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta227<F: Float>(t30: F, t1317: F, t1857: F, t1320: F, t1468: F, t3833: F, t2: F, t513: F, t580: F, t605: F, t1711: F, t3841: F, t516: F, zeta_threshold: F, t33: F, t1113: F, t162: F, t189: F, t512: F, t1856: F, t749: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5545, t5546, t5547, t5548, t5549, t5552, t5556, t5557, t5560) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1443::<F>(t30, t1317, t1857, t1320, t1468, t3833, t2, t513, t580, t605, t1711, t3841, t516, zeta_threshold);
        let t5566 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1444::<F>(t33, t1113, t5557, t5560, t580, t162, t5556, zeta_threshold);
        let t5567 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1445::<F>(t189, t5566);
        let (t5568, t5569) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1446::<F>(t512, t5567, t1856, t749);
    (t5545, t5546, t5547, t5548, t5549, t5552, t5557, t5560, t5566, t5567, t5568, t5569)
}
