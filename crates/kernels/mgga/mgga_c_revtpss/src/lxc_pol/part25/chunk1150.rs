//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1150/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1150<F: Float>(t3244: F, t7111: F, t3111: F, t7132: F, t1971: F, t3229: F, t351: F, t1058: F, t7126: F, t1973: F, t3201: F, t1020: F, t7125: F) -> (F, F, F, F, F, F, F) {
    let t25543 = t7111 * t3244;
    let t25551 = t7132 * t3111;
    let t25553 = t1971 * t3229;
    let t25554 = t351 * t25553;
    let t25557 = t7126 * t1058;
    let t25560 = F::new(0.95275595817932748827e-4) * t1973 * t3201;
    let t25561 = t1020 * t7125;
    (t25543, t25551, t25553, t25554, t25557, t25560, t25561)
}
