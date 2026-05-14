//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1321/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1321<F: Float>(t2350: F, t31058: F, t2256: F, t8268: F, t31026: F, t31028: F, t31030: F, t31033: F, t31035: F, t31036: F, t31040: F, t31044: F, t31047: F, t31051: F, t31055: F, t69: F, t8258: F, t8267: F) -> (F, F, F) {
    let t31059 = t31058 * t2350;
    let t31062 = t8268 * t2256;
    let t31065 = -t31026 - 4.0 / 3.0 * t31028 - 10.0 / 9.0 * t31030 + 10.0 / 9.0 * t31033 - 3.0 / 4.0 * t31035 * t31036 - 5.0 / 6.0 * t8258 * t31040 + 5.0 / 6.0 * t8258 * t31044 + t8258 * t31047 / 4.0 - 5.0 / 9.0 * t69 * t31051 + 25.0 / 36.0 * t8267 * t31055 - 5.0 / 36.0 * t8267 * t31059 - 5.0 / 24.0 * t8267 * t31062;
    (t31059, t31062, t31065)
}
