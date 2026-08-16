//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1366/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1366<F: Float>(t10525: F, t10526: F, t34239: F, t6689: F, t8411: F, t31590: F, t475: F) -> (F, F, F) {
    let t34318 = F::cast_from(0.42900587942220512002e1_f64) * t10525 * t10526 * t34239;
    let t34320 = F::cast_from(0.10725146985555128001e1_f64) * t8411 * t6689;
    let t34321 = t31590 * t475;
    (t34318, t34320, t34321)
}
