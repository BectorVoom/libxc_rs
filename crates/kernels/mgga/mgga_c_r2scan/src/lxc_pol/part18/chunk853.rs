//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 853/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk853<F: Float>(t5834: F, t5966: F, t5968: F, t5970: F, t5972: F, t5975: F, t5976: F, t5978: F, t5980: F, t5982: F, t5985: F, t7849: F) -> F {
    let t9025 = -t5966 + F::new(0.21687162600603479684e-1) * t5968 - F::new(0.32106488758451047386e0) * t5970 - F::new(0.1301229756036208781e0) * t5972 - t5975 + F::new(8.0) * t5976 - F::new(0.11290853155555555555e-2) * t5978 + t5834 + F::new(8.0) * t5980 - F::new(20.0) * t5982 + t5985 + t7849;
    t9025
}
