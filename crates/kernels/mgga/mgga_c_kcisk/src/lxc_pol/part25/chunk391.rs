//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 391/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk391<F: Float>(t1800: F, t2537: F, t1869: F, t1693: F, t1796: F, t2399: F, t2470: F, t2475: F, t2511: F, t2530: F, t2535: F, t671: F, t752: F, t196: F) -> (F, F, F, F, F) {
    let t2538 = t1800 * t2537;
    let t2539 = t1869 * t2538;
    let t2541 = t2399 * t671 - 0.193e0 * t1693 * t2470 + t1796 + 0.16581944444444444444e-2 * t2475 + 0.24872916666666666666e-2 * t2511 - 0.24872916666666666666e-2 * t2530 - 0.66327777777777777776e-2 * t2535 + 0.16581944444444444444e-2 * t2539;
    let t2542 = t2541 * t752;
    let t2543 = t2399 * t196;
    (t2538, t2539, t2541, t2542, t2543)
}
