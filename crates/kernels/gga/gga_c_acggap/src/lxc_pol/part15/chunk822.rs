//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 822/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk822<F: Float>(t1997: F, t3036: F, t3213: F, t1035: F, t1039: F, t7613: F, t30589: F, t7548: F, t2109: F, t7630: F, t2113: F, t30546: F, t7499: F, t2450: F, t7432: F) -> (F, F, F, F, F, F, F) {
    let t30904 = t3036 * t1997 * t3213;
    let t30907 = t1035 * t7613 * t1039;
    let t30920 = t30589 * t7548;
    let t30924 = t7630 * t2109;
    let t30926 = t7630 * t2113;
    let t30928 = t30546 * t7499;
    let t30934 = t2450 * t7432;
    (t30904, t30907, t30920, t30924, t30926, t30928, t30934)
}
