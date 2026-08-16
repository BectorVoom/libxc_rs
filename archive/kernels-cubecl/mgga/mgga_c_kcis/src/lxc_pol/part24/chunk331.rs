//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 331/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk331<F: Float>(t1029: F, t1030: F, t1697: F, t1717: F, t1728: F, t1745: F, t278: F, t305: F, t339: F) -> F {
    let t1747 = -t1029 - F::cast_from(0.23426533963880895498e-2_f64) * t1030 * t1717 - F::cast_from(0.46853067927761790996e-2_f64) * t305 * t1728 - t1697 * t339 - t278 * t1745;
    t1747
}
