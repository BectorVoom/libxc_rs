//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 861/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk861<F: Float>(t5378: F, t5391: F, t6622: F, t73: F, t5327: F, t5362: F, t1803: F, t5326: F, t5323: F, t12772: F, t6639: F, t3625: F, t1263: F, t6573: F, t1038: F, t6593: F) -> (F, F, F, F, F, F, F, F, F) {
    let t21001 = t5391 * t5378;
    let t21040 = t6622 * t73;
    let t21053 = t5327 * t5362;
    let t21063 = t5326 * t1803;
    let t21088 = t5323 * t5362;
    let t21090 = t12772 * t6639;
    let t21091 = t3625 * t21090;
    let t21093 = t1263 * t6573;
    let t21100 = t6593 * t1038;
    (t21001, t21040, t21053, t21063, t21088, t21090, t21091, t21093, t21100)
}
