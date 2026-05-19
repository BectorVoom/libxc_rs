//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 997/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk997<F: Float>(t322: F, t3675: F, t856: F, t1338: F, t3678: F, t11893: F, t1348: F, t11145: F, t11148: F, t11157: F, t11162: F, t11920: F, t11924: F, t11926: F, t11960: F, t11991: F, t2438: F, t330: F, t3413: F, t3420: F, t352: F, t3643: F, t3645: F, t837: F, t838: F, t855: F, t9760: F) -> (F, F, F, F, F) {
    let t323 = t322 <= F::new(0.0);
    let t331 = t322 <= F::new(0.25e1);
    let t332 = F::new(0.25e1) < t322;
    let t11993 = t3675 * t856;
    let t12002 = t1338 * t3678;
    let t12005 = piecewise3::<F>(t332, t11893, F::new(0.0));
    let t12009 = t1348 * t3678;
    let t12019 = piecewise5::<F>(t323, t330 * t3643 * t837 + t11920 * t330 + t11924 * t330 + t11926 * t330 + t3645 * t838, t331, t11960 + t11991, -F::new(0.63e1) * t3420 * t11993 - F::new(0.21e1) * t11145 * t3675 - F::new(0.945e1) * t11148 * t11993 - F::new(0.21e1) * t3413 * t9760 - F::new(0.21e1) * t12002 * t2438 - F::new(0.105e1) * t855 * t12005 * t352 - F::new(0.1575e1) * t12009 * t2438 - F::new(0.1575e1) * t11157 * t3675 - F::new(0.1575e1) * t3420 * t9760 - F::new(0.23625e1) * t11162 * t11993);
    (t11993, t12002, t12005, t12009, t12019)
}
