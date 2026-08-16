//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 765/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk765<F: Float>(t2049: F, t35253: F, t35688: F, t7760: F, t2019: F, t271: F, t3118: F, t641: F, t7491: F, t7927: F, t20: F, t2018: F, t2021: F, t4710: F) -> (F, F, F, F) {
    let t35691 = t35688 * t2049 * t35253 * t7760;
    let t35696 = t2019 * t3118 * t271 * t641;
    let t35698 = t7491 * t7927;
    let t35702 = t4710 * t20 * t2018 * t2021;
    (t35691, t35696, t35698, t35702)
}
