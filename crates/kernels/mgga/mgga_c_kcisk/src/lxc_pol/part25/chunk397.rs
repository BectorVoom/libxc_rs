//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 397/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk397<F: Float>(t2571: F, t735: F, t734: F, t2507: F, t716: F, t740: F, t748: F, t2527: F, t747: F, t746: F) -> (F, F, F, F, F, F) {
    let t2572 = t735 * t2571;
    let t2573 = t734 * t2572;
    let t2575 = t2507 * t716;
    let t2576 = t2575 * t740;
    let t2577 = t2576 * t748;
    let t2579 = t747 * t2527;
    let t2580 = t746 * t2579;
    (t2572, t2573, t2575, t2576, t2577, t2580)
}
