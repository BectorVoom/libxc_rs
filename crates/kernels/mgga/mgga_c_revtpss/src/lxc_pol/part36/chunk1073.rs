//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1073/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1073<F: Float>(t12382: F, t16706: F, t20283: F, t20285: F, t20287: F, t24230: F, t24234: F, t24238: F, t24242: F, t24246: F, t24250: F, t422: F) -> F {
    let t24466 = -t12382 + F::new(0.23744444444444444444e-1) * t16706 + F::new(0.11872222222222222222e-1) * t20283 - F::new(0.35616666666666666666e-1) * t20285 - F::new(0.17808333333333333333e-1) * t20287 + F::new(0.19787037037037037037e-1) * t24230 - F::new(0.71233333333333333332e-1) * t24234 - F::new(0.35616666666666666666e-1) * t24238 + F::new(0.10685e0) * t24242 + F::new(0.10685e0) * t24246 + F::new(0.17808333333333333333e-1) * t24250;
    let t24468 = F::new(0.621814e-1) * t24466 * t422;
    t24468
}
