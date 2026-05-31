//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1022/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1022<F: Float>(t22723: F, t1872: F, t2048: F, t120: F, t6916: F, t105: F, t635: F, t6990: F, t2024: F, t136: F, t634: F, t6922: F) -> (F, F, F, F, F, F) {
    let t22724 = F::cast_from(1440.0_f64) * t22723;
    let t22727 = t2048 * t1872;
    let t22728 = F::cast_from(192.0_f64) * t22727;
    let t22751 = t120 * t6916;
    let t22786 = t105 * t6990 * t635;
    let t22787 = t2024 * t2024;
    let t22797 = t634 * t6922 * t136;
    (t22724, t22728, t22751, t22786, t22787, t22797)
}
