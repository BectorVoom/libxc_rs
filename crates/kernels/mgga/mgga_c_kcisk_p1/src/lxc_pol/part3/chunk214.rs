//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 214/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk214<F: Float>(t110: F, t10: F, t107: F, t142: F, t64: F, t903: F, t41: F, t120: F, t117: F, t8: F) -> (F, F, F) {
    let t111 = t110 < -F::cast_from(0.66725e-1_f64);
    let t911 = piecewise3::<F>(t111, F::cast_from(0.0_f64), F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t64 * t903 * t10 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t64 * t107 * t142);
    let t912 = t911 * t41;
    let t913 = t912 * t120;
    let t916 = t117 * t8;
    (t912, t913, t916)
}
