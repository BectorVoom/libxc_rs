//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 780/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk780<F: Float>(t17: F, t8946: F, t8947: F, t120: F, t1570: F, t16: F, t2252: F, t341: F, t37820: F, t23: F, t32905: F, t153: F, t1984: F, t22: F, t36452: F, t37991: F) -> (F, F, F, F, F, F, F) {
    let t39926 = t8946 * t8947 * t17;
    let t39931 = t120 * t1570;
    let t39942 = t8946 * t16;
    let t39976 = t341 * t2252;
    let t40033 = 0.4939111192043895748e-1 * t37820;
    let t40266 = t23 * t32905;
    let t40280 = 1.0 / t153 / t37991 / t22 / t1984 / t36452 / 96.0;
    (t39926, t39931, t39942, t39976, t40033, t40266, t40280)
}
