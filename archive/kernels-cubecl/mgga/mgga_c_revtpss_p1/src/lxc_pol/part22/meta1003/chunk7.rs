//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3425/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3425<F: Float>(t15416: F, t4632: F, t15475: F, t4590: F, t41880: F, t6110: F, t41549: F, t51967: F, t63274: F, t63276: F, t63278: F, t63281: F, t63285: F, t63290: F, t63293: F, t63299: F, t63304: F, t63308: F) -> (F, F, F, F) {
    let t64342 = F::cast_from(4.0_f64) * t15416 * t4632;
    let t64344 = F::cast_from(2.0_f64) * t4590 * t15475;
    let t64346 = F::cast_from(2.0_f64) * t41880 * t6110;
    let t64358 = F::cast_from(0.71233333333333333332e-1_f64) * t63274 - F::cast_from(0.23744444444444444444e-1_f64) * t63276 + F::cast_from(0.79148148148148148146e-2_f64) * t63278 - F::cast_from(0.23744444444444444444e-1_f64) * t63281 - F::cast_from(0.11872222222222222222e-1_f64) * t63285 - F::cast_from(0.19787037037037037037e-1_f64) * t63290 + F::cast_from(0.71233333333333333332e-1_f64) * t63293 + F::cast_from(0.35616666666666666666e-1_f64) * t63299 + F::cast_from(0.23744444444444444444e0_f64) * t63304 - F::cast_from(0.42739999999999999999e0_f64) * t63308 + t41549 + F::cast_from(0.11872222222222222222e-1_f64) * t51967;
    (t64342, t64344, t64346, t64358)
}
