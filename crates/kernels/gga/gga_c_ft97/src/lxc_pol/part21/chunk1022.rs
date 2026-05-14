//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1022/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1022<F: Float>(t7837: F, t92439: F, t373: F, t5546: F, t8042: F, t92356: F, t1614: F, t391: F, t1624: F, t9: F, t173: F, t22584: F, t1602: F, t1711: F, t22582: F, t1630: F, t420: F, t422: F) -> (F, F, F, F, F, F, F) {
    let t92440 = t7837 * t92439;
    let t92441 = t5546 * t373;
    let t92456 = t8042 * t92356;
    let t92461 = t1614 * t391;
    let t92463 = t1624 * t9 * t92461;
    let t92466 = t173 * t22584;
    let t92470 = t1602 * t1711;
    let t92471 = t92470 * t22582;
    let t92476 = t420 * t422 * t1630;
    (t92440, t92441, t92456, t92463, t92466, t92471, t92476)
}
