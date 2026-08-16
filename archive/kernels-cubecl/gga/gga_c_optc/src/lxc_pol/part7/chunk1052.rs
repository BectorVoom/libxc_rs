//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1052/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1052<F: Float>(t2007: F, t6953: F, t627: F, t6896: F, t631: F, t130: F, t131: F, t142: F, t20816: F, t127: F, t2022: F, t2067: F) -> (F, F, F, F) {
    let t22844 = t2007 * t6953;
    let t22850 = t6896 * t627;
    let t22851 = t22850 * t631;
    let t22856 = F::cast_from(0.36717874996221960261e1_f64) * t130 * t131 * t20816 * t142;
    let t22858 = t2067 * t2022 * t127;
    (t22844, t22851, t22856, t22858)
}
