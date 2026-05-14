//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 530/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk530<F: Float>(t1191: F, t3671: F, t1172: F, t1170: F, t305: F) -> (F, F, F, F, F) {
    let t3672 = t3671 * t1191;
    let t3674 = 1.0 * t1172 * t3672;
    let t3675 = t1170 * t1170;
    let t3676 = 1.0 / t3675;
    let t3677 = t305 * t3676;
    (t3672, t3674, t3675, t3676, t3677)
}
