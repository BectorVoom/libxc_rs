//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 767/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk767<F: Float>(t302: F, t7350: F, t7349: F, t7353: F, t31: F, t35214: F, t7351: F, t35604: F, t2019: F, t2020: F, t7220: F, t7224: F) -> (F, F, F, F, F) {
    let t35718 = t7350 * t302;
    let t35720 = t7349 * t35718 * t7353;
    let t35724 = t7349 * t7351 * t35214 * t31;
    let t35728 = t7349 * t7351 * t35604 * t31;
    let t35731 = t2019 * t2020 * t7220;
    let t35737 = t2019 * t2020 * t7224;
    (t35720, t35724, t35728, t35731, t35737)
}
