//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1151/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1151<F: Float>(t52212: F, t52916: F, t66902: F, t66905: F, t66934: F, t66945: F, t67420: F, t80685: F, t80696: F, t80759: F, t80770: F, t80772: F, t88198: F, t88201: F, t88213: F) -> F {
    let t89513 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t80685 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t66902 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t66905 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t88198 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t88201 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t80696 + F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t66934 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t66945 + F::cast_from(112.0_f64) / F::cast_from(243.0_f64) * t52212 + F::cast_from(112.0_f64) / F::cast_from(81.0_f64) * t52916 - F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t80759 - F::cast_from(16.0_f64) / F::cast_from(81.0_f64) * t67420 + F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t80770 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t80772 + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t88213;
    t89513
}
