//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1145/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1145<F: Float>(t1052: F, t3147: F, t1036: F, t3141: F, t3229: F, t369: F, t361: F, t351: F, t3106: F, t3111: F, t3156: F, t3172: F) -> (F, F, F, F, F, F, F) {
    let t11997 = t1052 * t3147;
    let t11998 = t1036 * t11997;
    let t11999 = t3141 * t11998;
    let t12002 = t3229 * t369;
    let t12003 = t361 * t12002;
    let t12004 = t351 * t12003;
    let t12007 = t3106 * t3111;
    let t12009 = t3172 * t3156;
    (t11997, t11998, t11999, t12003, t12004, t12007, t12009)
}
