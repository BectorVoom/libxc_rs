//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta963 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3225;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta963<F: Float>(t10428: F, t5999: F, t18544: F, t2398: F, t14440: F, t4311: F, t4537: F, t775: F, t14386: F, t4308: F, t39860: F, t18498: F, t2403: F, t2404: F, t39799: F, t39807: F, t39813: F, t39818: F, t39823: F, t4541: F, t4556: F) -> (F, F, F, F, F, F) {
        let (t61177, t61179, t61181, t61190, t61191, t61192) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3225::<F>(t10428, t5999, t18544, t2398, t14440, t4311, t4537, t775, t14386, t4308, t39860, t18498, t2403, t2404, t39799, t39807, t39813, t39818, t39823, t4541, t4556);
    (t61177, t61179, t61181, t61190, t61191, t61192)
}
