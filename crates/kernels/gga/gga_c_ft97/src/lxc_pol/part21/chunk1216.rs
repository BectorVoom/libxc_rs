//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1216/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1216<F: Float>(t29904: F, t8392: F, t1882: F, t29813: F, t11837: F, t6557: F, t26113: F, t3238: F, t1825: F, t29789: F, t103654: F, t103783: F, t103796: F, t11593: F, t116324: F, t116328: F, t11863: F, t1339: F, t15885: F, t16151: F, t16320: F, t16324: F, t1901: F, t26349: F, t446: F, t452: F, t47659: F, t83: F, t91739: F, t93630: F, t93647: F) -> (F, F, F, F) {
    let t118137 = t8392 * t29904;
    let t118139 = t1882 * t29813;
    let t118141 = t11837 * t6557;
    let t118150 = t3238 * t26113;
    let t118154 = t1825 * t29789;
    let t118168 = 4.0 / 9.0 * t47659 * t91739 * t16320 + 4.0 / 9.0 * t47659 * t103654 * t16324 + t103783 - 2.0 / 27.0 * t118137 + 2.0 / 9.0 * t118139 - 2.0 / 3.0 * t446 * t83 * t118141 - 4.0 / 27.0 * t93630 - t446 * t452 * t1339 * t15885 / 3.0 - 2.0 / 3.0 * t446 * t83 * t118150 - t446 * t83 * t118154 / 3.0 + 4.0 / 9.0 * t1901 * t26349 * t16151 + 4.0 / 81.0 * t93647 - t103796 - 4.0 / 9.0 * t1901 * t11863 * t116324 - 8.0 / 9.0 * t11593 * t11863 * t116328;
    (t118141, t118150, t118154, t118168)
}
