//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1027/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1027<F: Float>(t146: F, t6533: F, t774: F, t110: F, t252: F, t6359: F, t545: F, t7613: F, t19790: F, t495: F, t1559: F, t283: F) -> (F, F, F, F, F) {
    let t22796 = t146 * t6533 * t774;
    let t22820 = t146 * t110 * t6359 * t252;
    let t22868 = t545 * t7613;
    let t22948 = t19790 * t495;
    let t23038 = t1559 * t1559;
    let t23040 = F::cast_from(1.0_f64) / t283 / t23038;
    (t22796, t22820, t22868, t22948, t23040)
}
