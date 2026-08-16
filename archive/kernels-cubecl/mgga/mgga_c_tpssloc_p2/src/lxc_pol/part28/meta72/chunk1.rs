//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 480/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk480<F: Float>(t1433: F, t72: F, t1411: F, t1427: F, t66: F, t80: F) -> (F, F) {
    let t1434 = t72 * t1433;
    let t1437 = -t1411 * t80 / F::cast_from(12.0_f64) + t1427 * t80 / F::cast_from(24.0_f64) + t66 * t1434 / F::cast_from(24.0_f64);
    (t1434, t1437)
}
