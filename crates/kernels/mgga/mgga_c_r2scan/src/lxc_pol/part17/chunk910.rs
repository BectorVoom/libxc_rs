//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 910/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk910<F: Float>(t322: F, t1338: F, t3774: F, t12240: F, t1348: F, t11302: F, t11305: F, t11314: F, t11319: F, t11993: F, t12267: F, t12271: F, t12273: F, t12307: F, t12338: F, t2438: F, t330: F, t352: F, t3549: F, t3556: F, t3675: F, t3740: F, t3742: F, t837: F, t838: F, t855: F, t9760: F) -> (F, F, F, F) {
    let t323 = t322 <= 0.0;
    let t331 = t322 <= 0.25e1;
    let t332 = 0.25e1 < t322;
    let t12348 = t1338 * t3774;
    let t12351 = piecewise3(t332, t12240, 0.0);
    let t12355 = t1348 * t3774;
    let t12365 = piecewise5(t323, t330 * t3740 * t837 + t12267 * t330 + t12271 * t330 + t12273 * t330 + t3742 * t838, t331, t12307 + t12338, -0.63e1 * t3556 * t11993 - 0.21e1 * t11302 * t3675 - 0.945e1 * t11305 * t11993 - 0.21e1 * t3549 * t9760 - 0.21e1 * t12348 * t2438 - 0.105e1 * t855 * t12351 * t352 - 0.1575e1 * t12355 * t2438 - 0.1575e1 * t11314 * t3675 - 0.1575e1 * t3556 * t9760 - 0.23625e1 * t11319 * t11993);
    (t12348, t12351, t12355, t12365)
}
