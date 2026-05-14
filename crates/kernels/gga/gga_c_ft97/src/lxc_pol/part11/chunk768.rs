//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 768/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk768<F: Float>(t37385: F, t13: F, t7741: F, t18: F, t7742: F) -> (F, F) {
    let t37386 = 8.0 / 27.0 * t37385;
    let t37387 = t7741 * t13;
    let t37388 = 1.0 / t37387;
    let t37389 = t18 * t37388;
    let t37391 = -24.0 * t7742 + 24.0 * t37389;
    (t37386, t37391)
}
