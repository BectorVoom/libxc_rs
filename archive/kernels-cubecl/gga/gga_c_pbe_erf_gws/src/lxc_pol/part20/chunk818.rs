//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 818/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk818<F: Float>(t2659: F, t586: F, t2816: F, t636: F, t197: F, t589: F, t172: F, t2824: F, t184: F, t2684: F, t5137: F, t639: F) -> (F, F, F, F, F) {
    let t7136 = t2659 * t586;
    let t7147 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t2816 * t636;
    let t7148 = t589 * t197;
    let t7170 = t172 * t2824;
    let t7171 = t7170 * t184;
    let t7188 = t5137 * t2684;
    let t7190 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t639 * t7188;
    (t7136, t7147, t7148, t7171, t7190)
}
