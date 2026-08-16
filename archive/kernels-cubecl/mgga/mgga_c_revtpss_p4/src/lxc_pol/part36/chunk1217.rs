//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1217/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1217<F: Float>(t93012: F, t2453: F, t2783: F, t64: F, t7030: F, t9784: F, t2482: F, t25260: F, t27: F, t596: F, t7036: F, t2681: F, t820: F) -> (F, F, F, F, F, F) {
    let t93013 = F::cast_from(0.22589491248727328397e-6_f64) * t93012;
    let t93015 = t2453 * t2783 * t64;
    let t93020 = t9784 * t7030;
    let t93021 = F::cast_from(0.14450132032386466905e-2_f64) * t93020;
    let t93025 = t2482 * t25260 * t27;
    let t93034 = t2482 * t7036 * t596;
    let t93048 = t820 * t7036 * t2681;
    (t93013, t93015, t93021, t93025, t93034, t93048)
}
