//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta291 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1534;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta291<F: Float>(t10428: F, t707: F, t2398: F, t2414: F, t10326: F, t190: F, t706: F, t2258: F, t750: F, t157: F, t36: F, t10356: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t10430, t10432, t10433, t10435, t10436, t10437, t10438, t10439, t10440) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1534::<F>(t10428, t707, t2398, t2414, t10326, t190, t706, t2258, t750, t157, t36, t10356);
    (t10430, t10432, t10433, t10435, t10436, t10437, t10438, t10439, t10440)
}
