//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 723/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk723<F: Float>(t13143: F, t13151: F, t13679: F, t13681: F, t13693: F, t13695: F, t13697: F, t13700: F, t13703: F, t13704: F, t13898: F, t13899: F) -> F {
    let t14406 = t13679 + t13681 - t13693 - t13695 + t13697 - t13898 + t13899 + t13700 + t13703 - F::new(0.44688112439813033337e-1) * t13704 + F::new(0.63904876589867916127e-1) * t13143 - F::new(0.63904876589867916127e-1) * t13151;
    t14406
}
