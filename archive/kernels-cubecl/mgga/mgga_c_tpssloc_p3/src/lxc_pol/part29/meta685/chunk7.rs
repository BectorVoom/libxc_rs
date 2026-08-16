//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2345/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2345<F: Float>(t1860: F, t2109: F, t2110: F, t22489: F, t22493: F, t22534: F, t24504: F, t24511: F, t26024: F, t27308: F, t27311: F, t6486: F, t7255: F, t7428: F, t7445: F, t7974: F, t7975: F, t7978: F, t90132: F, t90257: F) -> F {
    let t96209 = t22534 * t7975 / F::cast_from(3.0_f64) + t22534 * t7978 / F::cast_from(3.0_f64) + t90132 * t2110 / F::cast_from(3.0_f64) - t7428 * t24511 / F::cast_from(6.0_f64) - t22493 * t7975 / F::cast_from(6.0_f64) - t1860 * t7974 * t22489 / F::cast_from(6.0_f64) - t22493 * t7978 / F::cast_from(6.0_f64) - t6486 * t27308 / F::cast_from(3.0_f64) - t6486 * t27311 / F::cast_from(3.0_f64) - t1860 * t24504 * t7445 / F::cast_from(6.0_f64) - t1860 * t7255 * t26024 / F::cast_from(3.0_f64) - t1860 * t2109 * t90257 / F::cast_from(6.0_f64);
    t96209
}
