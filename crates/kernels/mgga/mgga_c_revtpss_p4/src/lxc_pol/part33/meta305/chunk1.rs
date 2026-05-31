//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1301/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1301<F: Float>(t2236: F, t3: F, t25: F, t2246: F, t599: F, t88: F, t89: F, t90: F, t29: F) -> (F, F, F, F) {
    let t10292 = t2236 * t3;
    let t10293 = F::cast_from(1.0_f64) / t10292;
    let t10295 = F::cast_from(336.0_f64) * t25 * t10293;
    let t10301 = t599 * t2246;
    let t10308 = F::cast_from(1.0_f64) / t90 / t89 / t88;
    let t10309 = t29 * t10308;
    (t10295, t10301, t10308, t10309)
}
