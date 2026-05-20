//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1050/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1050<F: Float>(t25304: F, t7283: F, t1426: F, t3999: F, t25821: F, t2106: F, t530: F, t10309: F, t7342: F, t38: F, t624: F, t2247: F) -> (F, F, F, F, F, F, F) {
    let t26069 = t25304 * t7283;
    let t26079 = t1426 * t3999;
    let t26148 = F::new(22.0) / F::new(9.0) * t25821;
    let t26161 = t530 * t2106;
    let t26175 = t10309 * t7342;
    let t26178 = t38 * t624;
    let t26179 = t2247 * t26178;
    (t26069, t26079, t26148, t26161, t26175, t26178, t26179)
}
