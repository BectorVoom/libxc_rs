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
    let t89513 = F::new(8.0) / F::new(3.0) * t80685 - F::new(8.0) / F::new(9.0) * t66902 + F::new(16.0) / F::new(9.0) * t66905 + F::new(8.0) / F::new(9.0) * t88198 - F::new(8.0) / F::new(3.0) * t88201 + F::new(8.0) / F::new(9.0) * t80696 + F::new(16.0) / F::new(27.0) * t66934 - F::new(8.0) / F::new(27.0) * t66945 + F::new(112.0) / F::new(243.0) * t52212 + F::new(112.0) / F::new(81.0) * t52916 - F::new(16.0) / F::new(27.0) * t80759 - F::new(16.0) / F::new(81.0) * t67420 + F::new(8.0) / F::new(27.0) * t80770 - F::new(8.0) / F::new(27.0) * t80772 + F::new(3.0) / F::new(4.0) * t88213;
    t89513
}
