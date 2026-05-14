//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 377/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk377<F: Float>(t1808: F, t1809: F, t2399: F, t2477: F, t2488: F, t2505: F, t604: F, t674: F, t702: F) -> (F,) {
    let t2507 = -t1808 - 0.23426533963880895498e-2 * t1809 * t2477 - 0.46853067927761790996e-2 * t674 * t2488 - t2399 * t702 - t604 * t2505;
    (t2507,)
}
