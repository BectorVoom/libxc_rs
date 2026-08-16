//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 869/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk869<F: Float>(t120: F, t1570: F, t16: F, t8946: F, t2252: F, t341: F, t37820: F, t23: F, t32905: F, t153: F, t1984: F, t22: F, t36452: F, t37991: F) -> (F, F, F, F, F, F) {
    let t39931 = t120 * t1570;
    let t39942 = t8946 * t16;
    let t39976 = t341 * t2252;
    let t40033 = F::cast_from(0.4939111192043895748e-1_f64) * t37820;
    let t40266 = t23 * t32905;
    let t40280 = F::cast_from(1.0_f64) / t153 / t37991 / t22 / t1984 / t36452 / F::cast_from(96.0_f64);
    (t39931, t39942, t39976, t40033, t40266, t40280)
}
