//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 748/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk748<F: Float>(t40802: F, t5148: F, t333: F, t8915: F, t4669: F, t128: F, t30526: F, t338: F, t6444: F, t39665: F, t5259: F, t38569: F, t7782: F, t321: F, t8712: F, t262: F) -> (F, F, F, F, F, F, F, F, F) {
    let t40803 = t5148 * t40802;
    let t40805 = t8915 * t333;
    let t40806 = t4669 * t40805;
    let t40823 = t30526 * t128;
    let t40826 = t6444 * t338;
    let t40831 = t5259 * t39665;
    let t40891 = t7782 * t38569;
    let t40897 = t8712 * t321;
    let t40898 = t262 * t40897;
    (t40803, t40805, t40806, t40823, t40826, t40831, t40891, t40897, t40898)
}
