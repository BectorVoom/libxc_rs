//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1323/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1323<F: Float>(t114111: F, t114125: F, t114136: F, t114139: F, t114209: F, t119124: F, t119128: F, t119131: F, t119141: F, t119144: F, t119149: F, t32019: F, t32022: F, t34697: F, t34715: F, t34744: F, t9796: F) -> (F,) {
    let t119151 = 0.14739506172839506172e-2 * t119124 + t114111 + 0.73697530864197530862e-3 * t119128 + t114125 - 0.26805555555555555557e-2 * t119131 + 0.10416666666666666667e-1 * t32019 * t34697 - 0.22109259259259259259e-2 * t114136 + t114139 - 0.20833333333333333334e-1 * t32019 * t34744 + 0.8041666666666666667e-2 * t114209 * t9796 - 0.44218518518518518517e-2 * t119141 - 0.33163888888888888888e-2 * t119144 - 0.18518518518518518519e-1 * t32022 * t34715 + 0.23148148148148148149e-2 * t119149;
    (t119151,)
}
