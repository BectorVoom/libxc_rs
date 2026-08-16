//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2137/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2137<F: Float>(t28184: F, t7235: F, t2014: F, t25190: F, t28176: F, t1907: F, t4135: F, t28196: F, t28197: F, t28173: F, t25188: F, t7901: F) -> (F, F, F, F, F) {
    let t98546 = F::cast_from(6.0_f64) * t7235 * t28184;
    let t98549 = F::cast_from(6.0_f64) * t2014 * t25190 * t28176;
    let t98550 = t1907 * t4135;
    let t98553 = F::cast_from(2.0_f64) * t28196 * t28197 * t98550;
    let t98555 = F::cast_from(6.0_f64) * t7235 * t28173;
    let t98557 = F::cast_from(3.0_f64) * t25188 * t7901;
    (t98546, t98549, t98553, t98555, t98557)
}
