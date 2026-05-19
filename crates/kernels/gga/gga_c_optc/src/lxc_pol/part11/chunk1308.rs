//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1308/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1308<F: Float>(t232: F, t57312: F, t57324: F, t57114: F, t7681: F, t799: F, t16867: F, t30827: F, t24699: F, t7672: F, t10493: F, t16875: F) -> (F, F, F, F, F) {
    let t57327 = F::new(0.62182e-1) * (t57312 + t57324) * t232;
    let t57330 = F::new(24.0) * t7681 * t57114 * t799;
    let t57332 = F::cast_from(0.38596378373162651572e3_f64) * t30827 * t16867;
    let t57335 = F::cast_from(0.620700176468474021e4_f64) * t24699 * t57114 * t7672;
    let t57337 = F::new(24.0) * t10493 * t16875;
    (t57327, t57330, t57332, t57335, t57337)
}
