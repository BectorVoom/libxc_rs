//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 988/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk988<F: Float>(t40273: F, t39761: F, t39767: F, t39772: F, t39776: F, t39781: F, t39784: F, t39788: F, t39792: F, t39796: F, t40265: F, t40270: F, t40288: F, t40292: F) -> F {
    let t40567 = F::new(56.0) / F::new(81.0) * t40273;
    let t40570 = F::new(2.0) / F::new(9.0) * t39761 + F::new(4.0) / F::new(3.0) * t39767 + t39772 - F::new(4.0) / F::new(3.0) * t39776 - F::new(40.0) / F::new(243.0) * t39781 + F::new(4.0) / F::new(9.0) * t39784 + F::new(4.0) / F::new(3.0) * t39788 + t39792 / F::new(3.0) - t39796 / F::new(9.0) - t40265 / F::new(6.0) + F::new(4.0) * t40270 + t40567 - F::new(6.0) * t40288 - t40292 / F::new(18.0);
    t40570
}
