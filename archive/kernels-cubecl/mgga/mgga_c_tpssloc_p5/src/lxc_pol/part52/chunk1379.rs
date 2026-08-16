//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1379/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1379<F: Float>(t119877: F, t120002: F, t120003: F, t123119: F, t123120: F, t123122: F, t123124: F, t123126: F, t123129: F, t24999: F, t26103: F, t27879: F, t6517: F, t7271: F, t7989: F, t8329: F) -> F {
    let t123137 = -F::cast_from(2.0_f64) * t24999 * t7271 - F::cast_from(2.0_f64) * t26103 * t7989 - F::cast_from(2.0_f64) * t27879 * t6517 + t119877 + t120002 - t120003 - t123119 - F::cast_from(2.0_f64) * t123120 - F::cast_from(2.0_f64) * t123122 - F::cast_from(2.0_f64) * t123124 - F::cast_from(2.0_f64) * t123126 - F::cast_from(2.0_f64) * t123129 - t8329;
    t123137
}
