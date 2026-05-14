//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 944/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk944<F: Float>(t105: F, t635: F, t6990: F, t2024: F, t136: F, t634: F, t6922: F, t137: F, t627: F, t6896: F, t130: F, t131: F, t142: F, t20816: F, t2003: F, t2010: F) -> (F, F, F, F, F, F, F) {
    let t22786 = t105 * t6990 * t635;
    let t22787 = t2024 * t2024;
    let t22797 = t634 * t6922 * t136;
    let t22834 = t137 * t137;
    let t22835 = 1.0 / t22834;
    let t22836 = t136 * t22835;
    let t22850 = t6896 * t627;
    let t22856 = 0.36717874996221960261e1 * t130 * t131 * t20816 * t142;
    let t22889 = t2003 * t2010;
    (t22786, t22787, t22797, t22836, t22850, t22856, t22889)
}
