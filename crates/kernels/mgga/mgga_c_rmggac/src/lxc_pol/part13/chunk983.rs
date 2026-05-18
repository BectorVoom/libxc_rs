//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 983/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk983<F: Float>(t27059: F, t3351: F, t3352: F, t515: F, t2019: F, t2020: F, t8858: F, t2010: F, t2012: F, t5757: F, t4962: F, t8854: F) -> (F, F, F, F, F) {
    let t41600 = t3351 * t3352 * t515 * t27059;
    let t41604 = t2019 * t2020 * t8858;
    let t41607 = t2010 * t2012 * t5757;
    let t41610 = t2010 * t2012 * t4962;
    let t41613 = t2019 * t2020 * t8854;
    (t41600, t41604, t41607, t41610, t41613)
}
