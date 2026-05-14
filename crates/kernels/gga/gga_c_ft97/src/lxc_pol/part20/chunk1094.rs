//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1094/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1094<F: Float>(t108817: F, t14116: F, t2441: F, t6034: F, t6804: F, t96690: F, t17836: F, t52: F, t6018: F, t14075: F, t2446: F, t6035: F, t13863: F, t9657: F, t24378: F, t27651: F, t27653: F) -> (F, F, F, F, F, F) {
    let t108819 = t108817 * t2441 * t14116;
    let t108823 = t6034 * t96690 * t6804;
    let t108826 = t17836 * t6018 * t52;
    let t108830 = t6035 * t2446 * t14075;
    let t108834 = t6035 * t9657 * t13863;
    let t108838 = t27651 * t24378 * t27653;
    (t108819, t108823, t108826, t108830, t108834, t108838)
}
