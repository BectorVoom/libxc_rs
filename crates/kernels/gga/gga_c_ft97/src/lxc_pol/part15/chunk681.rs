//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 681/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk681<F: Float>(t20045: F, t378: F, t92: F, t11167: F, t15734: F, t15750: F, t15760: F, t20025: F, t20029: F, t20033: F, t20037: F, t20041: F, t7945: F) -> (F, F, F) {
    let t20046 = t378 * t20045;
    let t20047 = t92 * t20046;
    let t20049 = -t7945 - F::new(4.0) / F::new(9.0) * t11167 + F::new(2.0) / F::new(9.0) * t15734 - F::new(2.0) / F::new(3.0) * t15750 + t15760 / F::new(3.0) - F::new(10.0) / F::new(27.0) * t20025 + F::new(4.0) / F::new(3.0) * t20029 - F::new(2.0) / F::new(3.0) * t20033 - F::new(2.0) * t20037 + F::new(2.0) * t20041 - t20047 / F::new(3.0);
    (t20046, t20047, t20049)
}
