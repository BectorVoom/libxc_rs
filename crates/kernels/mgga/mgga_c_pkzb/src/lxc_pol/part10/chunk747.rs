//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 747/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk747<F: Float>(t158: F, t3428: F, t3429: F, t1746: F, t3401: F, t3396: F, t596: F, t1029: F, t1031: F, t160: F, t162: F) -> (F, F, F, F) {
    let t3431 = (t3428 + t3429) * t158;
    let t3435 = t1746 * t3401;
    let t3438 = t596 * t3396;
    let t3441 = 6.0 * t1029 * t1031 - 12.0 * t160 * t3435 + 3.0 * t160 * t3438 - t162 * t3431;
    (t3431, t3435, t3438, t3441)
}
