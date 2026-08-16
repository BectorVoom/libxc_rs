//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta135 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk714;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk715;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk716;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk717;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta135<F: Float>(t1450: F, t1907: F, t198: F, t530: F, t532: F, t1317: F, t1857: F, t1320: F, t1468: F, t3833: F, t1711: F, t3841: F, t1856: F, t749: F, t512: F, t177: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t5532 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk714::<F>(t1450, t1907);
        let t5536 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk715::<F>(t198, t530);
        let (t5541, t5545, t5547, t5549, t5557, t5569) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk716::<F>(t198, t532, t1317, t1857, t1320, t1468, t3833, t1711, t3841, t1856, t749);
        let (t5570, t5571) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk717::<F>(t512, t5569, t177, t1856);
    (t5532, t5536, t5541, t5545, t5547, t5549, t5557, t5569, t5570, t5571)
}
