//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1101/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1101<F: Float>(t10852: F, t2253: F, t170: F, t328: F, t39600: F, t10850: F, t10904: F, t10915: F, t14514: F, t14519: F, t2265: F, t231: F, t2928: F, t2938: F, t2939: F, t2951: F, t41448: F, t41468: F, t43046: F, t43050: F, t43062: F, t43072: F, t43074: F, t43076: F, t43078: F, t631: F, t898: F) -> F {
    let t43080 = t2253 * t10852;
    let t43084 = F::cast_from(220.0_f64) / F::cast_from(81.0_f64) * t170 * t39600 * t328;
    let t43088 = F::cast_from(8.0_f64) * t2265 * t14519 * t43046 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t631 * t10915 * t43050 * t41448 - F::cast_from(4.0_f64) * t631 * t231 * t10850 * t41448 - t631 * t231 * t2928 * t41468 - F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t631 * t898 * t2938 * t43062 + F::cast_from(36.0_f64) * t631 * t898 * t10904 * t2939 * t2951 - F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t43072 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t43074 - F::cast_from(16.0_f64) / F::cast_from(81.0_f64) * t43076 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t43078 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t43080 - t43084 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t2265 * t14514 * t43046;
    t43088
}
