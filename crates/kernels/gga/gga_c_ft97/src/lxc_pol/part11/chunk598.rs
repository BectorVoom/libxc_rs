//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 598/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk598<F: Float>(t1526: F, t1944: F, t7705: F, t1948: F, t342: F, t630: F, t142: F, t1557: F, t1559: F, t1570: F, t1580: F, t1943: F, t2075: F, t72: F, t1527: F, t1953: F, t1970: F, t2081: F, t3088: F, t343: F, t8759: F) -> (F, F, F, F, F, F) {
    let t8761 = t1526 * t7705 * t1944;
    let t8764 = t342 * t630 * t1948;
    let t8766 = t142 * t1557;
    let t8767 = t8766 * t1559;
    let t8774 = t142 * t1570;
    let t8775 = t8774 * t1559;
    let t8779 = t1943 * t1580;
    let t8783 = t72 * t2075;
    let t8787 = t1953 + t2081 + t8759 - t8761 / 18.0 - t8764 / 6.0 - t1526 * t3088 * t8767 / 9.0 - t1526 * t1527 * t1970 / 6.0 + t1526 * t1527 * t8775 / 6.0 - t1526 * t1527 * t8779 / 12.0 - t342 * t343 * t8783 / 4.0;
    (t8766, t8767, t8775, t8779, t8783, t8787)
}
