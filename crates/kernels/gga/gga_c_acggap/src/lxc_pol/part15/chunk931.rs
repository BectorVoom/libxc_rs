//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 931/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk931<F: Float>(t151: F, t7731: F, t950: F, t3378: F, t7560: F, t30049: F, t7461: F, t2104: F, t7610: F, t1113: F, t7736: F, t377: F, t7732: F) -> (F, F, F, F, F, F) {
    let t31811 = t151 * t7731 * t950;
    let t31824 = t3378 * t7560;
    let t31839 = t30049 * t7461;
    let t31849 = t7610 * t2104;
    let t31855 = t7736 * t1113;
    let t31863 = t377 * t7732;
    (t31811, t31824, t31839, t31849, t31855, t31863)
}
