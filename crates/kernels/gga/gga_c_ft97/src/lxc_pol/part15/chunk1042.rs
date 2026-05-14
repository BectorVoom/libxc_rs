//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1042/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1042<F: Float>(t13605: F, t1526: F, t20489: F, t21181: F, t21931: F, t21958: F, t21969: F, t21973: F, t22168: F, t2320: F, t2639: F, t3806: F, t44700: F, t44716: F, t72992: F, t82488: F, t82491: F) -> (F,) {
    let t90537 = -t1526 * t2320 * t21969 / 4.0 - t1526 * t2320 * t2639 * t20489 / 12.0 + t22168 - t1526 * t3806 * t21958 / 3.0 - 7.0 / 27.0 * t1526 * t13605 * t44700 * t21181 - t1526 * t2320 * t21973 / 4.0 - t82488 / 9.0 - t82491 / 6.0 + t21931 - t44716 + t72992 / 18.0;
    (t90537,)
}
