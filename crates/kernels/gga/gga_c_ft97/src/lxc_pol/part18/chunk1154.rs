//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1154/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1154<F: Float>(t25846: F, t28: F, t469: F, t473: F, t5665: F, t1755: F, t6454: F, t1317: F, t1800: F, t22862: F, t942: F, t11449: F, t5691: F, t446: F, t7824: F, t11827: F) -> (F, F, F, F, F, F, F, F) {
    let t100333 = t5665 * t28 * t469 * t25846 * t473;
    let t100335 = t6454 * t1755;
    let t100338 = t1317 * t28 * t1800 * t100335;
    let t100340 = t22862 * t942;
    let t100343 = t1317 * t28 * t1800 * t100340;
    let t100345 = t5691 * t11449;
    let t100347 = t446 * t7824 * t100345;
    let t100349 = t5691 * t11827;
    (t100333, t100335, t100338, t100340, t100343, t100345, t100347, t100349)
}
