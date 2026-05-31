//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1653/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1653<F: Float>(t290: F, t2925: F, t11300: F, t11385: F, t3022: F, t3030: F, t3034: F, t3006: F, t3011: F, t4733: F, t981: F, t2935: F, t945: F) -> (F, F, F, F, F, F, F, F) {
    let t11387 = F::cast_from(1.0_f64) / t2925 / t290;
    let t11388 = t11300 * t11387;
    let t11390 = F::cast_from(0.51726012919273400301e3_f64) * t11385 * t11388;
    let t11392 = F::cast_from(0.17544670867903938621e1_f64) * t3022 * t3030;
    let t11394 = F::cast_from(0.51947577317044391276e2_f64) * t3022 * t3034;
    let t11396 = t3011 * t3006 * t4733;
    let t11398 = F::cast_from(0.51947577317044391277e2_f64) * t981 * t11396;
    let t11399 = t2935 * t945;
    (t11387, t11388, t11390, t11392, t11394, t11396, t11398, t11399)
}
