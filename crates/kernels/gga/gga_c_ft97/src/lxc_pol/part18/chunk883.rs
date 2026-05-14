//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 883/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk883<F: Float>(t23610: F, t586: F, t23609: F, t28: F, t376: F, t5890: F, t5892: F, t23510: F, t9073: F, t446: F, t2112: F, t23527: F, t1369: F, t23518: F, t5905: F, t5842: F, t590: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t23611 = t586 * t23610;
    let t23613 = t23609 * t28 * t23611;
    let t23616 = t5890 * t376 * t5892;
    let t23618 = t9073 * t23510;
    let t23619 = t446 * t23618;
    let t23621 = t2112 * t23527;
    let t23623 = t1369 * t28 * t23621;
    let t23625 = t2112 * t23518;
    let t23627 = t1369 * t28 * t23625;
    let t23629 = t1369 * t376 * t5905;
    let t23631 = t5842 * t590;
    (t23611, t23613, t23616, t23618, t23619, t23621, t23623, t23625, t23627, t23629, t23631)
}
