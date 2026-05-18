//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 555/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk555<F: Float>(t2013: F, t7487: F, t1297: F, t20: F, t2018: F) -> (F, F, F) {
    let t7488 = t7487 * t2013;
    let t7489 = F::new(0.19211284388664477842e-2) * t7488;
    let t7490 = t1297 * t20;
    let t7491 = t7490 * t2018;
    (t7489, t7490, t7491)
}
