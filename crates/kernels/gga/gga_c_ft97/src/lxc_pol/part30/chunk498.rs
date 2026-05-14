//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 498/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk498<F: Float>(t339: F, t39: F, t11: F, t340: F, t14: F, t1526: F, t4906: F, t9483: F, t10915: F, t240: F, t3691: F, t2917: F, t3700: F, t18: F, t2321: F, t342: F, t4910: F, t630: F) -> (F, F, F, F, F, F) {
    let t15564 = t339 * t39;
    let t15565 = t340 * t11;
    let t15567 = t15564 * t15565 * t14;
    let t17685 = t1526 * t9483 * t4906;
    let t17687 = t10915 * t240;
    let t17688 = t17687 * t3691;
    let t17694 = t2917 * t240;
    let t17695 = t17694 * t3700;
    let t17698 = t2321 * t18;
    let t17703 = t342 * t630 * t4910;
    (t15567, t17685, t17688, t17695, t17698, t17703)
}
