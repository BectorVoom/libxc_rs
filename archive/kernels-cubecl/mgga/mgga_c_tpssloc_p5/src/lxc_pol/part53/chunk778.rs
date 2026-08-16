//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 778/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk778<F: Float>(t192: F, t531: F, t1982: F, t25: F, t870: F, t4255: F, t16596: F, t22960: F, t1484: F, t606: F, t4119: F, t7484: F, t794: F) -> (F, F, F, F, F, F, F) {
    let t24994 = t192 * t531;
    let t24995 = t1982 * t24994;
    let t25014 = t870 * t25;
    let t25015 = t25014 * t4255;
    let t25021 = t22960 * t16596;
    let t25024 = t606 * t1484;
    let t25028 = t25 * t4119;
    let t25035 = t794 * t7484;
    (t24994, t24995, t25015, t25021, t25024, t25028, t25035)
}
