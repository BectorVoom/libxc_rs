//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1127/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1127<F: Float>(t6034: F, t6804: F, t96690: F, t17836: F, t52: F, t6018: F, t24378: F, t27651: F, t27653: F, t13580: F, t24389: F, t3750: F, t24361: F, t27647: F, t27671: F, t420: F, t55105: F) -> (F, F, F, F, F, F) {
    let t108823 = t6034 * t96690 * t6804;
    let t108826 = t17836 * t6018 * t52;
    let t108838 = t27651 * t24378 * t27653;
    let t108845 = t13580 * t24389 * t3750;
    let t108857 = t24361 * t24378 * t27647;
    let t108860 = t27671 * t420 * t55105;
    (t108823, t108826, t108838, t108845, t108857, t108860)
}
