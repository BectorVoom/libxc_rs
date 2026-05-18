//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1112/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1112<F: Float>(t7535: F, t9593: F, t116: F, t28651: F, t2106: F, t47672: F, t2097: F, t9990: F, t3999: F, t7506: F, t198: F, t7443: F) -> (F, F, F, F, F, F) {
    let t102005 = t7535 * t9593;
    let t102019 = t28651 * t116;
    let t102070 = t2106 * t47672;
    let t102397 = t9990 * t2097;
    let t102622 = t3999 * t7506;
    let t102851 = t198 * t7443;
    (t102005, t102019, t102070, t102397, t102622, t102851)
}
