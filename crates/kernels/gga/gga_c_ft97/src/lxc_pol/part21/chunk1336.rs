//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1336/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1336<F: Float>(t119546: F, t119550: F, t120110: F, t120112: F, t120115: F, t120117: F, t120120: F, t120171: F, t120219: F, t120271: F, t120324: F, t120335: F, t120364: F, t120411: F, t120458: F, t120497: F, t120549: F, t120600: F, t120647: F, t120696: F, t120748: F, t120786: F, t120839: F, t121064: F, t121072: F, t121122: F, t121155: F, t121193: F, t121220: F, t121251: F, t121284: F, t121341: F, t121379: F, t121404: F, t121432: F, t121462: F, t121505: F, t121541: F, t121570: F, t121604: F, t121639: F, t149: F, t160: F, t1969: F, t24102: F, t26546: F, t4458: F, t4650: F, t4720: F, t5772: F, t5775: F, t5973: F, t6580: F) -> (F,) {
    let t121649 = t5772 * t1969 * t24102 * t4458 / 9.0 - t119546 * t5775 / 18.0 - t4650 * t5973 - 2.0 * t119550 - 2.0 / 3.0 * t6580 * t26546 - 2.0 * t120110 - 2.0 * t120112 - t4720 * t5973 - 2.0 * t120115 - 4.0 * t120117 - t120120 / 18.0 - t149 * (t121072 + t120171 + t120696 + t121155 + t120748 + t121193 + t120324 + t121220 + t121341 + t120411 + t120458 + t120647 + t120786 + t121404 + t120271 + t121432 + t121462 + t121639 + t121251 + t120364 + t121505 + t120497 + t120600 + t121604 + t121570 + t121541 + t121122 + t121284 + t120549 + t120219 + t121379 + t120839) + 4.0 * t120335 + 2.0 * t121064 * t160;
    (t121649,)
}
