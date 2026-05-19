//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 793/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk793<F: Float>(t114: F, t4287: F, t655: F, t2335: F, t2336: F, t4261: F, t4264: F, t69: F) -> (F, F) {
    let t115 = F::new(1.0) < t114;
    let t4288 = t655 * t4287;
    let t4292 = piecewise3::<F>(t115, F::new(0.0), t2335 + t2336 / F::new(3.0) + t4261 / F::new(3.0) + t69 * t4264 / F::new(4.0) - t69 * t4288 / F::new(8.0));
    (t4288, t4292)
}
