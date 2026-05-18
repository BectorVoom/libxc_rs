//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 857/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk857<F: Float>(t2209: F, t3255: F, t6188: F, t772: F, t3243: F, t10110: F, t3247: F, t277: F, t6851: F, t8449: F, t7108: F, t8452: F, t959: F) -> (F, F, F, F, F, F) {
    let t10278 = t2209 * t3255;
    let t10280 = t772 * t6188;
    let t10281 = t3243 * t10280;
    let t10284 = t10110 * t3247;
    let t10286 = t277 * t6851;
    let t10287 = t8449 * t10286;
    let t10289 = t8452 * t959 * t7108;
    (t10278, t10281, t10284, t10286, t10287, t10289)
}
