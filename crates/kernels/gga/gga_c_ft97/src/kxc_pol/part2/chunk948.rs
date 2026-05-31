//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 948/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk948<F: Float>(t14699: F, t193: F, t89: F, t1212: F, t2682: F, t7640: F, t10400: F, t10279: F, t1186: F, t9733: F, t13730: F, t4044: F) -> (F, F, F, F, F, F) {
    let t14701 = t89 * t193 * t14699;
    let t14704 = t7640 * t1212 * t2682;
    let t14706 = t89 * t193 * t14704;
    let t14708 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t10400;
    let t14711 = F::cast_from(4.0_f64) / F::cast_from(81.0_f64) * t10279;
    let t14715 = t89 * t9733 * t1186;
    let t14718 = t89 * t13730 * t4044;
    (t14701, t14706, t14708, t14711, t14715, t14718)
}
