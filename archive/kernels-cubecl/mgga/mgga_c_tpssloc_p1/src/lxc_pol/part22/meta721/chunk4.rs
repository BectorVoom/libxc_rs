//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2348/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2348<F: Float>(t16662: F, t221: F, t4127: F, t4128: F, t46790: F, t46794: F, t46796: F, t46806: F, t46856: F, t59195: F, t68110: F, t68116: F, t68118: F, t68122: F) -> F {
    let t68124 = F::cast_from(0.14999999999999999999e-1_f64) * t4127 * t221 * t4128 * t16662 - F::cast_from(0.74999999999999999995e-2_f64) * t68110 + F::cast_from(0.16851851851851851851e0_f64) * t46790 + t46794 + F::cast_from(0.47499999999999999999e-1_f64) * t46796 + F::cast_from(0.8333333333333333333e-3_f64) * t46806 - F::cast_from(0.38888888888888888887e-1_f64) * t59195 - t46856 + F::cast_from(0.38888888888888888887e-2_f64) * t68116 + F::cast_from(0.46666666666666666664e-1_f64) * t68118 + F::cast_from(0.99999999999999999995e-2_f64) * t68122;
    t68124
}
