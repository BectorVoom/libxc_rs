//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 381/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk381<F: Float>(t3094: F, t3107: F, t3099: F, t3104: F, t471: F, t871: F, t984: F, t3114: F) -> (F, F, F) {
    let t3330 = F::cast_from(3.0_f64) / F::cast_from(128.0_f64) * t3094;
    let t3333 = t3107 / F::cast_from(128.0_f64);
    let t3334 = t3330 - F::cast_from(9.0_f64) / F::cast_from(4096.0_f64) * t3099 + F::cast_from(3.0_f64) / F::cast_from(4096.0_f64) * t3104 - t3333;
    let t3335 = t3334 * t471;
    let t3336 = t984 * t871;
    let t3338 = t3335 + t3336 / F::cast_from(2.0_f64) + t3330 - t3333 - t3114;
    (t3334, t3336, t3338)
}
