//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 949/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk949<F: Float>(t26153: F, t508: F, t2106: F, t530: F, t25865: F, t6977: F, t7348: F, t1923: F, t2047: F, t25146: F, t10309: F, t7342: F, t38: F, t624: F, t2247: F, t6960: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26154 = t508 * t26153;
    let t26161 = t530 * t2106;
    let t26162 = t26161 * t25865;
    let t26169 = t7348 * t6977;
    let t26170 = t1923 * t26169;
    let t26172 = t2047 * t25146;
    let t26175 = t10309 * t7342;
    let t26178 = t38 * t624;
    let t26179 = t2247 * t26178;
    let t26180 = t26179 * t6960;
    (t26154, t26162, t26169, t26170, t26172, t26175, t26178, t26179, t26180)
}
