//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 511/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk511<F: Float>(t586: F, t4636: F, t1670: F, t45: F, t1675: F, t596: F) -> (F, F, F, F, F) {
    let t4743 = t586 * t586;
    let t4744 = 1.0 / t4743;
    let t4748 = 0.12361111111111111111e-1 * t4636;
    let t4757 = t45 * t1670;
    let t4760 = t1675 * t596;
    let t4761 = 1.0 / t4760;
    (t4743, t4744, t4748, t4757, t4761)
}
