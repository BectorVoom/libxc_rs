//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1440/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1440<F: Float>(t3447: F, t4900: F, t4904: F, t64821: F, t73169: F, t73330: F, t73386: F, t73389: F, t73395: F, t73417: F, t73420: F, t73424: F, t78031: F, t78039: F) -> F {
    let t78460 = F::cast_from(0.11111111111111111111e-2_f64) * t3447 * t73169 * t4904 - F::cast_from(0.22222222222222222221e-2_f64) * t73330 + F::cast_from(0.88888888888888888887e-2_f64) * t73386 - F::cast_from(0.11111111111111111111e-2_f64) * t73389 + F::cast_from(0.11111111111111111111e-2_f64) * t73395 - F::cast_from(0.14814814814814814815e-2_f64) * t73417 + F::cast_from(0.11111111111111111111e-2_f64) * t73420 - F::cast_from(0.74074074074074074072e-3_f64) * t64821 + F::cast_from(0.88888888888888888887e-2_f64) * t73424 + F::cast_from(0.14814814814814814815e-2_f64) * t3447 * t4900 * t78031 + F::cast_from(0.13333333333333333333e-1_f64) * t3447 * t4900 * t78039;
    t78460
}
