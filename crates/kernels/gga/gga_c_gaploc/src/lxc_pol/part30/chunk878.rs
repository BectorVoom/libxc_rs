//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 878/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk878<F: Float>(t10148: F, t10181: F, t10230: F, t10279: F, t209: F, t3362: F, t501: F, t605: F, t8042: F, t921: F, t2358: F, t8045: F, t2497: F, t2798: F, t1016: F, t6553: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10281 = t10148 + t10181 + t10230 + t10279;
    let t10282 = t10281 * t209;
    let t10283 = t3362 * t501;
    let t10284 = t10283 * t605;
    let t10285 = t8042 * t921;
    let t10286 = t8045 * t2358;
    let t10287 = 2.0 * t10286;
    let t10288 = t2798 * t2497;
    let t10289 = t6553 * t1016;
    (t10281, t10282, t10283, t10284, t10285, t10286, t10287, t10288, t10289)
}
