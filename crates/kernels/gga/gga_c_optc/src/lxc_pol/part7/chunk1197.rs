//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1197/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1197<F: Float>(t2418: F, t24303: F, t7669: F, t2409: F, t2416: F, t7681: F, t799: F, t2373: F, t2372: F, t2449: F, t2375: F, t2441: F, t7505: F) -> (F, F, F, F, F, F) {
    let t24708 = F::cast_from(0.57894567559743977359e3_f64) * t7669 * t24303 * t2418;
    let t24709 = t2409 * t2409;
    let t24712 = F::cast_from(0.48245472966453314466e2_f64) * t2416 * t24709 * t2418;
    let t24715 = F::cast_from(24.0_f64) * t7681 * t24303 * t799;
    let t24718 = F::cast_from(6.0_f64) * t2373 * t24709 * t799;
    let t24719 = t2449 * t2372;
    let t24721 = F::cast_from(12.0_f64) * t24719 * t2375;
    let t24723 = F::cast_from(0.41015588084031179722e4_f64) * t2441 * t7505;
    (t24708, t24712, t24715, t24718, t24721, t24723)
}
