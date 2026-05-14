//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1220/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1220<F: Float>(t1882: F, t29810: F, t29865: F, t10969: F, t25595: F, t29817: F, t103905: F, t103918: F, t103920: F, t103936: F, t110: F, t11593: F, t116365: F, t16266: F, t1871: F, t1901: F, t1902: F, t26356: F, t29839: F, t3052: F, t3266: F, t3291: F, t379: F, t39120: F, t446: F, t452: F, t5710: F, t6454: F, t6564: F, t83: F, t93828: F) -> (F, F) {
    let t118308 = t1882 * t29810;
    let t118310 = t1882 * t29865;
    let t118325 = t10969 * t25595;
    let t118337 = t1882 * t29817;
    let t118339 = -2.0 / 3.0 * t446 * t452 * t3291 * t6454 + t118308 / 9.0 - 2.0 / 9.0 * t118310 - t103905 - 4.0 / 27.0 * t93828 + 4.0 / 3.0 * t446 * t1871 * t110 * t116365 - t103918 - t103920 + 4.0 / 3.0 * t446 * t1871 * t6564 * t3266 + t103936 + 4.0 / 9.0 * t11593 * t1902 * t26356 * t3052 + 4.0 / 3.0 * t446 * t83 * t118325 + 2.0 / 9.0 * t1901 * t39120 * t29839 * t379 + t446 * t452 * t5710 * t16266 / 3.0 + t118337 / 9.0;
    (t118325, t118339)
}
