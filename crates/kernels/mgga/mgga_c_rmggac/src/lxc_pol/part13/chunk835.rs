//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 835/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk835<F: Float>(t352: F, t8915: F, t5148: F, t333: F, t4669: F, t2392: F, t876: F, t27048: F, t305: F, t38812: F, t128: F, t30526: F, t8645: F, t338: F, t6444: F, t8649: F) -> (F, F, F, F, F, F, F, F, F) {
    let t40802 = t8915 * t352;
    let t40803 = t5148 * t40802;
    let t40805 = t8915 * t333;
    let t40806 = t4669 * t40805;
    let t40808 = t2392 * t876;
    let t40809 = t27048 * t40808;
    let t40814 = t305 * t38812;
    let t40823 = t30526 * t128;
    let t40824 = t40823 * t8645;
    let t40826 = t6444 * t338;
    let t40827 = t40826 * t8649;
    (t40802, t40803, t40805, t40806, t40808, t40809, t40814, t40824, t40827)
}
