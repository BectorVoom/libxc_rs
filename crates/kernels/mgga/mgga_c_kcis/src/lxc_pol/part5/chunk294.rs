//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 294/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk294<F: Float>(t1029: F, t1030: F, t1031: F, t1046: F, t1083: F, t278: F, t305: F, t339: F, t975: F) -> F {
    let t1085 = -t1029 - F::new(0.23426533963880895498e-2) * t1030 * t1031 - F::new(0.46853067927761790996e-2) * t305 * t1046 - t975 * t339 - t278 * t1083;
    t1085
}
