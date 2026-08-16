//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta717 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2476;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta717<F: Float>(t48299: F, t123: F, t2630: F, t5566: F, t13665: F, t9863: F, t9866: F, t47101: F, t9575: F, t9572: F, t1320: F, t13680: F) -> (F, F, F, F, F, F, F, F) {
        let (t48300, t48303, t48304, t48306, t48312, t48313, t48324, t48326) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2476::<F>(t48299, t123, t2630, t5566, t13665, t9863, t9866, t47101, t9575, t9572, t1320, t13680);
    (t48300, t48303, t48304, t48306, t48312, t48313, t48324, t48326)
}
