//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 600/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk600<F: Float>(t2372: F, t4716: F, t1648: F, t1653: F, t6771: F, t2378: F, t827: F) -> (F, F, F, F) {
    let t6817 = t4716 * t2372;
    let t6818 = t6817 * t1648;
    let t6820 = t1653 * t6771;
    let t6823 = t827 * t2378;
    (t6817, t6818, t6820, t6823)
}
