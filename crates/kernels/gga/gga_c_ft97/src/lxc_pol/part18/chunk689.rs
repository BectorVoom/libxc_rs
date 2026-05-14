//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 689/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk689<F: Float>(t487: F, t942: F, t1876: F, t11810: F, t1820: F, t3238: F, t83: F, t1882: F, t3231: F, t11593: F, t11596: F, t11601: F, t11606: F, t11610: F, t11612: F, t11615: F, t11620: F, t11625: F, t11628: F, t11632: F, t11803: F, t11807: F, t1901: F, t28: F, t446: F, t89: F) -> (F, F) {
    let t11811 = t487 * t942;
    let t11812 = t11811 * t1876;
    let t11813 = t11810 * t11812;
    let t11816 = t3238 * t1820;
    let t11817 = t83 * t11816;
    let t11821 = 2.0 / 9.0 * t1882 * t3231;
    let t11822 = 4.0 / 9.0 * t11593 * t11596 + 4.0 / 9.0 * t11593 * t11601 + 8.0 / 9.0 * t11593 * t11606 + t11610 - t11612 - 2.0 / 3.0 * t446 * t11615 - 2.0 * t446 * t11620 - 2.0 / 3.0 * t446 * t11625 - 2.0 * t446 * t11628 - t11632 + t89 * t28 * t11803 / 3.0 + 2.0 / 9.0 * t1901 * t11807 - 4.0 / 3.0 * t1901 * t11813 - t446 * t11817 / 3.0 + t11821;
    (t11812, t11822)
}
