//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 887/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk887<F: Float>(t30: F, t525: F, t3834: F, t605: F, t3833: F, t2: F, t22: F, t580: F) -> (F, F, F, F, F, F) {
    let t9335 = F::new(1.0) / t525 / t30;
    let t9336 = t3834 * t605;
    let t9339 = t3833 * t605;
    let t9342 = t2 * t22;
    let t9343 = t580 - t9342;
    let t9344 = F::new(6.0) * t9343;
    (t9335, t9336, t9339, t9342, t9343, t9344)
}
