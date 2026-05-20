//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta998 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3389;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3390;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta998<F: Float>(t11294: F, t19331: F, t19127: F, t2926: F, t2924: F, t934: F, t3007: F, t6226: F, t981: F, t4631: F, t15543: F, t4719: F, t1634: F, t52877: F, t63597: F, t11299: F, t2875: F, t6110: F, t15101: F, t15383: F, t63633: F, t63636: F, t63638: F, t63641: F, t63644: F, t63647: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t63649, t63653, t63656, t63657, t63660, t63662) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3389::<F>(t11294, t19331, t19127, t2926, t2924, t934, t3007, t6226, t981, t4631, t15543, t4719);
        let (t63665, t63668, t63670, t63671) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3390::<F>(t1634, t52877, t63597, t11299, t2875, t6110, t15101, t15383, t63633, t63636, t63638, t63641, t63644, t63647, t63649, t63653, t63656, t63660, t63662);
    (t63649, t63653, t63656, t63657, t63660, t63662, t63665, t63668, t63670, t63671)
}
