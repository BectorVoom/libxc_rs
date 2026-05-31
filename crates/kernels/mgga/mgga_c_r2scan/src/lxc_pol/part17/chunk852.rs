//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 852/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk852<F: Float>(t5834: F, t5966: F, t5968: F, t5970: F, t5972: F, t5975: F, t5976: F, t5978: F, t5980: F, t5982: F, t5985: F, t7849: F) -> F {
    let t9025 = -t5966 + F::cast_from(0.21687162600603479684e-1_f64) * t5968 - F::cast_from(0.32106488758451047386e0_f64) * t5970 - F::cast_from(0.1301229756036208781e0_f64) * t5972 - t5975 + F::cast_from(8.0_f64) * t5976 - F::cast_from(0.11290853155555555555e-2_f64) * t5978 + t5834 + F::cast_from(8.0_f64) * t5980 - F::cast_from(20.0_f64) * t5982 + t5985 + t7849;
    t9025
}
