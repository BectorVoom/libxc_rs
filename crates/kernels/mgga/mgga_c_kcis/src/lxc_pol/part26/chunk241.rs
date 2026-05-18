//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 241/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk241<F: Float>(t1378: F, t1380: F, t286: F, t1367: F, t1368: F, t1373: F, t493: F) -> (F, F, F) {
    let t1381 = t1378 * t1380;
    let t1382 = t286 * t1381;
    let t1385 = t1367 + t1368 * t1373 / F::new(288.0) - t493 * t1382 / F::new(96.0);
    (t1381, t1382, t1385)
}
