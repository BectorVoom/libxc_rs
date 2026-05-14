//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 792/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk792<F: Float>(t10309: F, t7342: F, t38: F, t624: F, t2247: F, t6960: F, t2047: F, t25163: F, t6963: F, t7349: F, t10301: F, t6954: F, t239: F, t72: F, t1927: F, t1923: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26175 = t10309 * t7342;
    let t26178 = t38 * t624;
    let t26179 = t2247 * t26178;
    let t26180 = t26179 * t6960;
    let t26182 = t2047 * t25163;
    let t26185 = t6963 * t7349;
    let t26187 = t10301 * t7342;
    let t26190 = t6954 * t7349;
    let t26204 = t239 * t72;
    let t26205 = t26204 * t1927;
    let t26207 = 88.0 / 27.0 * t1923 * t26205;
    (t26175, t26178, t26179, t26180, t26182, t26185, t26187, t26190, t26207)
}
