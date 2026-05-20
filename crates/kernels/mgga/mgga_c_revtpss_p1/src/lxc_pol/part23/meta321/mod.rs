//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta321 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1610;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta321<F: Float>(t13665: F, t2630: F, t1857: F, t3860: F, t3863: F, t5566: F, t749: F, t512: F, t9856: F, t1468: F, t9605: F, t2: F, t3874: F) -> (F, F, F, F, F, F, F, F) {
        let (t13666, t13668, t13670, t13680, t13682, t13683, t13687, t13690) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1610::<F>(t13665, t2630, t1857, t3860, t3863, t5566, t749, t512, t9856, t1468, t9605, t2, t3874);
    (t13666, t13668, t13670, t13680, t13682, t13683, t13687, t13690)
}
