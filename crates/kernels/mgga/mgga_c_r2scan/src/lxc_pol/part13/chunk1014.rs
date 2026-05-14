//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1014/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1014<F: Float>(t1615: F, t3320: F, t783: F, t978: F, t261: F, t3299: F, t7291: F, t3594: F, t37736: F, t10879: F, t11741: F, t37660: F, t39540: F, t39542: F, t39545: F, t39549: F, t39550: F, t39552: F, t39554: F) -> (F,) {
    let t39558 = t783 * t978 * t1615 * t3320;
    let t39561 = t3299 * t261 * t7291;
    let t39563 = t37736 * t3594;
    let t39565 = t10879 * t11741;
    let t39567 = -0.43663693315433241792e-2 * t39540 - 0.16463622957338778997e0 * t39542 - 0.2600466522016280569e0 * t39545 - 0.14282990759302185292e-1 * t37660 - t39549 - 0.54878743191129263322e-1 * t39550 - 0.43341108700271342816e-1 * t39552 - 0.86682217400542685632e-1 * t39554 - 0.22511059664845582436e0 * t39558 - 0.43341108700271342816e-1 * t39561 - 0.13002332610081402845e0 * t39563 - 0.2600466522016280569e0 * t39565;
    (t39567,)
}
