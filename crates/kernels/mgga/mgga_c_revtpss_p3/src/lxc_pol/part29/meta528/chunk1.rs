//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1857/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1857<F: Float>(t26169: F, t6954: F, t1923: F, t26204: F, t6977: F, t25117: F, t7349: F, t1927: F, t72: F, t843: F, t26205: F, t45958: F, t7342: F) -> (F, F, F, F, F, F) {
    let t95243 = t6954 * t26169;
    let t95246 = t1923 * t26204 * t6977;
    let t95248 = t25117 * t7349;
    let t95253 = F::new(1232.0) / F::new(81.0) * t1923 * t843 * t72 * t1927;
    let t95255 = t6954 * t26205;
    let t95259 = t45958 * t7342;
    (t95243, t95246, t95248, t95253, t95255, t95259)
}
