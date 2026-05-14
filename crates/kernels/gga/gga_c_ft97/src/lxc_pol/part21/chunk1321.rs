//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1321/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1321<F: Float>(t106830: F, t106837: F, t106840: F, t106842: F, t106844: F, t106847: F, t107420: F, t107542: F, t13140: F, t17081: F, t17086: F, t17099: F, t1901: F, t2185: F, t26999: F, t27334: F, t27335: F, t30357: F, t3478: F, t3483: F, t3590: F, t446: F, t558: F, t574: F, t5855: F, t605: F, t6630: F, t6718: F, t9438: F, t95738: F, t95740: F) -> (F,) {
    let t121155 = 2.0 * t1901 * t13140 * t27335 * t17081 + 8.0 * t1901 * t27334 * t107542 * t17086 - 2.0 * t1901 * t26999 * t5855 * t17099 + t446 * t574 * t605 * t30357 * t558 / 3.0 + 4.0 / 3.0 * t446 * t2185 * t3590 * t6630 - t106830 - t106837 - t106840 - t106842 - t106844 + t106847 - 4.0 / 27.0 * t95738 - 4.0 / 27.0 * t95740 - 4.0 / 3.0 * t1901 * t13140 * t107420 * t3478 - 4.0 * t1901 * t27334 * t9438 * t6718 * t3483;
    (t121155,)
}
