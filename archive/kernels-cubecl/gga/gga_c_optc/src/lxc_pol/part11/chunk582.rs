//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 582/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk582<F: Float>(t4776: F, t767: F, t11: F, t2422: F, t3640: F, t4770: F, t4774: F) -> (F, F, F) {
    let t4777 = t767 * t4776;
    let t4778 = t11 * t4777;
    let t4780 = t2422 + F::cast_from(0.61805555555555555556e-2_f64) * t3640 - F::cast_from(0.61805555555555555555e-2_f64) * t4770 + F::cast_from(0.18541666666666666667e-1_f64) * t4774 - F::cast_from(0.92708333333333333333e-2_f64) * t4778;
    (t4777, t4778, t4780)
}
