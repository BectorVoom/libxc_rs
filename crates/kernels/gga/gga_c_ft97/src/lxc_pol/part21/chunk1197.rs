//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1197/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1197<F: Float>(t1882: F, t29841: F, t10969: F, t25598: F, t102878: F, t102880: F, t102882: F, t102903: F, t102917: F, t103864: F, t11593: F, t11854: F, t15772: F, t1901: F, t1902: F, t1909: F, t26267: F, t26367: F, t26390: F, t29605: F, t29918: F, t3052: F, t3238: F, t379: F, t38866: F, t446: F, t452: F, t47831: F, t5630: F, t59631: F, t6534: F, t83: F, t925: F) -> (F, F) {
    let t117330 = t1882 * t29841;
    let t117336 = t10969 * t25598;
    let t117355 = t1901 * t1902 * t5630 * t15772 / 9.0 + 2.0 / 27.0 * t1901 * t38866 * t29918 - 4.0 / 3.0 * t1901 * t59631 * t26367 + 2.0 / 9.0 * t117330 - t102878 - t102880 - t102882 + 2.0 / 9.0 * t1901 * t1902 * t103864 * t925 + 4.0 / 3.0 * t446 * t83 * t117336 + 2.0 / 3.0 * t446 * t452 * t3238 * t26390 - t102903 + 4.0 / 9.0 * t11593 * t1909 * t26267 * t3052 + 2.0 / 9.0 * t1901 * t47831 * t6534 + t102917 - 4.0 / 9.0 * t1901 * t11854 * t29605 * t379;
    (t117336, t117355)
}
