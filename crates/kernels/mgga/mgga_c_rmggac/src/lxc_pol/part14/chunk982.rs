//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 982/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk982<F: Float>(t1652: F, t7567: F, t352: F, t8915: F, t5148: F, t333: F, t4669: F, t2392: F, t876: F, t27048: F, t551: F, t7858: F) -> (F, F, F, F, F, F, F, F) {
    let t40791 = t7567 * t1652;
    let t40802 = t8915 * t352;
    let t40803 = t5148 * t40802;
    let t40804 = F::new(0.15965655602485078085e0) * t40803;
    let t40805 = t8915 * t333;
    let t40806 = t4669 * t40805;
    let t40807 = F::new(0.23948483403727617128e0) * t40806;
    let t40808 = t2392 * t876;
    let t40809 = t27048 * t40808;
    let t40811 = t7858 * t551;
    (t40791, t40802, t40804, t40805, t40807, t40808, t40809, t40811)
}
