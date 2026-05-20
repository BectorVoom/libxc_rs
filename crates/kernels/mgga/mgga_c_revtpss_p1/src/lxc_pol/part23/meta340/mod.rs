//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta340 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1642;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta340<F: Float>(t3920: F, t5603: F, t2435: F, t5718: F, t1893: F, t2453: F, t3908: F, t1904: F, t3895: F, t2439: F, t213: F, t5710: F) -> (F, F, F, F, F, F, F) {
        let (t14280, t14290, t14293, t14294, t14296, t14297, t14299) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1642::<F>(t3920, t5603, t2435, t5718, t1893, t2453, t3908, t1904, t3895, t2439, t213, t5710);
    (t14280, t14290, t14293, t14294, t14296, t14297, t14299)
}
