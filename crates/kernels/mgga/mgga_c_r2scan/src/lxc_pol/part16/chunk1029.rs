//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1029/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1029<F: Float>(t3269: F, t42838: F, t10615: F, t12395: F, t3262: F, t11545: F, t40276: F, t983: F, t986: F, t792: F, t40594: F, t4176: F, t39299: F, t3275: F, t3276: F, t12428: F, t481: F) -> (F, F, F, F, F, F, F) {
    let t42840 = 45.0 / 64.0 * t3269 * t42838;
    let t42843 = 15.0 / 8.0 * t3262 * t10615 * t12395;
    let t42845 = 5.0 / 8.0 * t40276 * t11545;
    let t42846 = t983 * t986;
    let t42847 = t42846 * t792;
    let t42850 = 45.0 / 32.0 * t40594 * t4176 * t42847;
    let t42851 = t39299 * t986;
    let t42854 = 5.0 / 8.0 * t3275 * t3276 * t42851;
    let t42855 = t12428 * t481;
    (t42840, t42843, t42845, t42846, t42850, t42854, t42855)
}
