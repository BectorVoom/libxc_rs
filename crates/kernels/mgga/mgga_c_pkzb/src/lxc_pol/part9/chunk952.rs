//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 952/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk952<F: Float>(t7335: F, t5522: F, t7332: F, t7352: F, t7361: F, t7363: F, t7366: F, t7368: F, t7371: F, t7373: F, t7376: F, t7379: F) -> F {
    let t7420 = F::cast_from(0.59793333333333333334e0_f64) * t7335;
    let t7431 = F::cast_from(0.27385555555555555555e0_f64) * t7332 - t7420 + F::new(0.8969e0) * t7352 + F::new(0.3071625e0) * t7361 + F::new(0.1898925e1) * t7363 - F::new(0.1898925e1) * t7366 - F::new(0.9494625e0) * t7368 + F::new(0.3071625e0) * t7371 + F::new(0.15358125e0) * t7373 + F::cast_from(0.142419375e1_f64) * t7376 - F::new(0.76790625e-1) * t7379 + F::cast_from(0.79724444444444444446e0_f64) * t5522;
    t7431
}
