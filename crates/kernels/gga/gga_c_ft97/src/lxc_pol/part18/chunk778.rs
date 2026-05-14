//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 778/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk778<F: Float>(t81: F, t8633: F, t2984: F, t2258: F, t2993: F, t1528: F, t18: F, t342: F, t4410: F, t630: F, t3103: F, t72: F, t11280: F, t1526: F, t1527: F, t15562: F, t15567: F, t2976: F, t2988: F, t3009: F, t3109: F, t343: F, t7704: F, t7707: F, t7710: F) -> (F, F, F, F, F, F, F) {
    let t15568 = t8633 * t81;
    let t15569 = t15568 * t2984;
    let t15575 = t2258 * t81;
    let t15576 = t15575 * t2993;
    let t15579 = t1528 * t18;
    let t15584 = t342 * t630 * t4410;
    let t15589 = t72 * t3103;
    let t15593 = t2976 + t3109 + t7704 - t7707 / 36.0 - t7710 / 12.0 - t15562 / 36.0 - t15567 * t15569 / 9.0 - t1526 * t1527 * t2988 / 12.0 + t15567 * t15576 / 6.0 - t1526 * t11280 * t15579 / 6.0 - t15584 / 12.0 - t1526 * t1527 * t3009 / 12.0 - t342 * t343 * t15589 / 4.0;
    (t15568, t15569, t15575, t15576, t15579, t15589, t15593)
}
