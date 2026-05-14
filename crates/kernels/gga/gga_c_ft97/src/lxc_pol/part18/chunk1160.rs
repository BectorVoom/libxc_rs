//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1160/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1160<F: Float>(t1564: F, t446: F, t920: F, t93440: F, t25846: F, t358: F, t363: F, t1588: F, t38921: F, t6469: F, t23050: F, t925: F, t93378: F, t93379: F, t1786: F, t5675: F) -> (F, F, F, F, F) {
    let t100438 = t446 * t1564 * t93440 * t920;
    let t100440 = t25846 * t358;
    let t100443 = t446 * t1564 * t100440 * t363;
    let t100447 = t446 * t38921 * t6469 * t1588;
    let t100451 = t93378 * t93379 * t925 * t23050;
    let t100453 = t1786 * t5675;
    (t100438, t100443, t100447, t100451, t100453)
}
