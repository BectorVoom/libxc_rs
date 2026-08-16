//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 946/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk946<F: Float>(t14237: F, t16503: F, t2281: F, t7482: F, t352: F, t8915: F, t5148: F, t333: F, t4669: F, t2392: F, t876: F, t27048: F) -> (F, F, F, F, F, F, F) {
    let t40780 = t16503 * t14237 * t2281 * t7482;
    let t40802 = t8915 * t352;
    let t40803 = t5148 * t40802;
    let t40805 = t8915 * t333;
    let t40806 = t4669 * t40805;
    let t40808 = t2392 * t876;
    let t40809 = t27048 * t40808;
    (t40780, t40802, t40803, t40805, t40806, t40808, t40809)
}
