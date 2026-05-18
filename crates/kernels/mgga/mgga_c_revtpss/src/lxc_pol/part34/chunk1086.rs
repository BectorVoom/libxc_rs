//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1086/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1086<F: Float>(t2275: F, t43: F, t239: F, t10309: F, t6957: F, t1962: F, t198: F, t206: F) -> (F, F, F, F) {
    let t25132 = t43 * t2275;
    let t25137 = F::new(88.0) / F::new(9.0) * t239;
    let t25157 = t10309 * t6957;
    let t25206 = t198 * t206 * t1962;
    (t25132, t25137, t25157, t25206)
}
