//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 445/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk445<F: Float>(t2374: F, t2375: F, t200: F, t262: F, t123: F, t126: F, t131: F, t119: F, t132: F, t63: F, t204: F, t686: F) -> (F, F, F, F, F, F, F) {
    let t2377 = F::cast_from(0.10843581300301739842e-1_f64) * t2374 * t2375;
    let t2378 = t200 * t262;
    let t2385 = F::cast_from(1.0_f64) / t126 / t123 * t131;
    let t2386 = t132 * t119;
    let t2387 = t2386 * t63;
    let t2388 = t2385 * t2387;
    let t2390 = t686 * t204;
    (t2377, t2378, t2385, t2386, t2387, t2388, t2390)
}
