//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 776/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk776<F: Float>(t1969: F, t34846: F, t7345: F, t7927: F, t35207: F, t7354: F, t2019: F, t2165: F, t7926: F, t2169: F, t7334: F, t7932: F) -> (F, F, F, F, F, F) {
    let t36772 = t34846 * t1969;
    let t36796 = t7345 * t7927;
    let t36797 = F::new(0.12195059916630011326e-2) * t36796;
    let t36801 = t35207 * t7354;
    let t36802 = F::new(0.5854811038705731867e-3) * t36801;
    let t36804 = t2019 * t7926 * t2165;
    let t36809 = t2019 * t7926 * t2169;
    let t36912 = t7334 * t7932;
    (t36772, t36797, t36802, t36804, t36809, t36912)
}
