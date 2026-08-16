//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 474/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk474<F: Float>(t2455: F, t2457: F, t2454: F, t252: F, t867: F, t786: F) -> (F, F, F, F) {
    let t2458 = t2455 * t2457;
    let t2460 = F::cast_from(0.11565819519348392139e-2_f64) * t2454 * t2458;
    let t2464 = t252 * t867;
    let t2465 = t786 * t2464;
    (t2458, t2460, t2464, t2465)
}
