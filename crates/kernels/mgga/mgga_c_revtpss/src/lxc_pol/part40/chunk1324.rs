//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1324/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1324<F: Float>(t114: F, t2358: F, t31149: F, t2362: F, t8315: F, t31035: F, t31134: F, t31135: F, t31137: F, t31139: F, t31143: F, t31146: F, t8258: F, t8267: F) -> (F, F, F) {
    let t115 = 1.0 < t114;
    let t31150 = t31149 * t2358;
    let t31153 = t8315 * t2362;
    let t31157 = piecewise3(t115, 0.0, -t31134 - 4.0 / 3.0 * t31135 + 10.0 / 9.0 * t31137 - 3.0 / 4.0 * t31035 * t31139 + 5.0 / 6.0 * t8258 * t31143 + t8258 * t31146 / 4.0 - 5.0 / 36.0 * t8267 * t31150 - 5.0 / 24.0 * t8267 * t31153);
    (t31150, t31153, t31157)
}
