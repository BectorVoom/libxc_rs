//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1072/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1072<F: Float>(t3088: F, t4166: F, t4183: F, t3378: F, t4176: F, t3077: F, t4163: F, t1035: F, t1647: F, t3044: F, t1655: F, t848: F) -> (F, F, F, F, F) {
    let t19048 = t3088 * t4166 * t4183;
    let t19053 = t3378 * t4176;
    let t19060 = t3077 * t4163;
    let t19074 = t1035 * t1647 * t3044;
    let t19082 = t848 * t1655;
    (t19048, t19053, t19060, t19074, t19082)
}
