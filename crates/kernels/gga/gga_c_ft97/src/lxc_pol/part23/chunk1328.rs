//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1328/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1328<F: Float>(t31627: F, t684: F, t113800: F, t113807: F, t113809: F, t113816: F, t113831: F, t11593: F, t125636: F, t125745: F, t1476: F, t15195: F, t15254: F, t15299: F, t18497: F, t1901: F, t19862: F, t2749: F, t29154: F, t29202: F, t29293: F, t296: F, t31862: F, t4129: F, t4246: F, t446: F, t5393: F, t6260: F, t7124: F, t840: F, t871: F) -> (F, F) {
    let t126368 = t31627 * t684;
    let t126372 = 8.0 / 9.0 * t11593 * t15254 * t29202 * t18497 + 8.0 / 81.0 * t113800 + t113807 + t113809 - t113816 + 2.0 / 3.0 * t446 * t296 * t125745 + t446 * t840 * t871 * t1476 * t19862 / 3.0 + t446 * t840 * t2749 * t31862 / 3.0 + t446 * t840 * t871 * t6260 * t5393 / 3.0 - 4.0 / 9.0 * t11593 * t15195 * t29154 - t113831 + 2.0 / 3.0 * t446 * t840 * t871 * t7124 * t4129 + 2.0 / 3.0 * t446 * t840 * t4246 * t29293 - 2.0 * t446 * t296 * t125636 - 2.0 / 9.0 * t1901 * t15299 * t126368;
    (t126368, t126372)
}
