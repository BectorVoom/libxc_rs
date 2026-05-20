//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta566 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2409;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta566<F: Float>(t5883: F, t648: F, t1501: F, t670: F, t6765: F, t1843: F, t4292: F, t1310: F, t5920: F, t116: F, t5876: F) -> (F, F, F, F, F, F) {
        let (t18220, t18227, t18232, t18235, t18242, t18245) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2409::<F>(t5883, t648, t1501, t670, t6765, t1843, t4292, t1310, t5920, t116, t5876);
    (t18220, t18227, t18232, t18235, t18242, t18245)
}
