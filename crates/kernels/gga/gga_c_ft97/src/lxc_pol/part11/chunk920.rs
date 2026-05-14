//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 920/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk920<F: Float>(t235: F, t9680: F, t226: F, t2428: F, t2393: F, t688: F, t13521: F, t2455: F, t709: F, t9548: F, t2395: F, t2417: F, t13468: F, t13474: F, t17958: F, t224: F, t2378: F, t2387: F, t2388: F, t2426: F, t2427: F, t3761: F, t3789: F, t41497: F, t41542: F, t678: F, t680: F, t695: F, t807: F, t9524: F, t9601: F, t9609: F, t9617: F, t9677: F) -> (F, F, F) {
    let t41547 = 1.0 / t9680 / t235;
    let t41548 = t226 * t41547;
    let t41549 = t2428 * t2428;
    let t41557 = t2393 * t688;
    let t41561 = t13521 * t2455;
    let t41569 = t2455 * t2455;
    let t41573 = t9548 * t709;
    let t41577 = t2395 * t2417;
    let t41588 = -t224 * t695 * (t41497 + t41542) + 24.0 * t224 * t41548 * t41549 - 0.23238868087529279928e-2 * t13468 * t2378 * t2417 * t2388 - 0.279058811357253504e-1 * t13474 * t41557 * t9617 - 0.279058811357253504e0 * t17958 * t3761 * t41561 + 8.0 * t3789 * t2426 * t9677 * t709 + 6.0 * t224 * t2427 * t41569 + 0.1116235245429014016e-1 * t2387 * t9609 * t41573 - 0.19352371901929178119e-4 * t678 * t807 * t41577 - 0.69716604262587839785e-3 * t678 * t9524 * t41577 + 0.46509801892875584e-1 * t2387 * t680 * t9601 * t709;
    (t41573, t41577, t41588)
}
