//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1396/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1396<F: Float>(t110092: F, t110097: F, t110099: F, t110106: F, t110341: F, t110778: F, t114783: F, t114784: F, t114790: F, t114794: F, t114796: F, t114799: F, t114803: F, t32026: F, t32180: F, t33346: F, t33384: F, t9796: F) -> (F,) {
    let t114805 = 0.18518518518518518519e-1 * t110778 + 0.88437037037037037034e-2 * t110092 - 0.88437037037037037034e-2 * t110097 + 0.1621345679012345679e-1 * t110099 - 0.73697530864197530861e-3 * t110106 + t114783 + 0.23148148148148148149e-2 * t114784 + 0.8041666666666666667e-2 * t32026 * t33346 + t114790 - 0.55555555555555555558e-1 * t110341 * t9796 - 0.24872916666666666666e-2 * t114794 + 0.22109259259259259258e-2 * t114796 + t114799 + 0.10416666666666666667e-1 * t33384 * t32180 - 0.23148148148148148149e-2 * t114803;
    (t114805,)
}
