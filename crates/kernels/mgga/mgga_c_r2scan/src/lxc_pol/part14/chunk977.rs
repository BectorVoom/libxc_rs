//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 977/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk977<F: Float>(t11397: F, t11413: F, t11430: F, t11447: F, t797: F, t1048: F, t499: F, t11017: F, t10634: F, t3472: F, t3262: F, t11011: F, t3465: F) -> (F, F, F, F, F, F, F) {
    let t11449 = t11397 + t11413 + t11430 + t11447;
    let t11450 = t11449 * t797;
    let t11452 = t1048 * t499 * t11450;
    let t11453 = t11452 / F::new(4.0);
    let t11454 = F::cast_from(0.39032073591371545778e-3_f64) * t11017;
    let t11455 = t3472 * t10634;
    let t11456 = t3262 * t11455;
    let t11457 = F::new(15.0) / F::new(8.0) * t11456;
    let t11458 = t3465 * t11011;
    (t11449, t11450, t11453, t11454, t11455, t11457, t11458)
}
