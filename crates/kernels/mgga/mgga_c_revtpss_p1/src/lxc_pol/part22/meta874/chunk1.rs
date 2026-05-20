//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3038/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3038<F: Float>(t10538: F, t51297: F, t14662: F, t251: F, t213: F, t225: F, t40321: F, t14939: F, t822: F, t686: F, t72: F, t874: F) -> (F, F, F, F, F) {
    let t51298 = t51297 * t10538;
    let t51306 = t251 * t14662;
    let t51320 = t213 * t225 * t40321;
    let t51332 = t822 * t14939;
    let t51339 = t874 * t14939 * t72 * t686;
    (t51298, t51306, t51320, t51332, t51339)
}
