//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta828 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2947;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta828<F: Float>(t13665: F, t9863: F, t9866: F, t9575: F, t9572: F, t1320: F, t13680: F, t3863: F, t5569: F, t3860: F, t5571: F, t9419: F) -> (F, F, F, F, F, F, F, F) {
        let (t48304, t48306, t48313, t48324, t48326, t48331, t48333, t48335) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2947::<F>(t13665, t9863, t9866, t9575, t9572, t1320, t13680, t3863, t5569, t3860, t5571, t9419);
    (t48304, t48306, t48313, t48324, t48326, t48331, t48333, t48335)
}
