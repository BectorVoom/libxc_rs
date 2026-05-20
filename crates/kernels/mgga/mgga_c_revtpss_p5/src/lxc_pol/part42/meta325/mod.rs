//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta325 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1111;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1112;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta325<F: Float>(t1385: F, t5710: F, t1904: F, t3899: F, t689: F, t3920: F, t5603: F, t2435: F, t5718: F, t1893: F, t2453: F, t3908: F, t3895: F, t2439: F, t213: F, t1532: F, t2609: F, t2398: F, t4305: F, t177: F, t4392: F, t762: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t14255, t14276, t14280, t14290, t14294) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1111::<F>(t1385, t5710, t1904, t3899, t689, t3920, t5603, t2435, t5718, t1893, t2453, t3908);
        let (t14297, t14299, t14312, t14317, t14324) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1112::<F>(t1904, t3895, t2439, t213, t5710, t1532, t2609, t2398, t4305, t177, t4392, t762);
    (t14255, t14276, t14280, t14290, t14294, t14297, t14299, t14312, t14317, t14324)
}
