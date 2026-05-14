//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1351/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1351<F: Float>(t2219: F, t7250: F, t2670: F, t6458: F, t19872: F, t7926: F, t1543: F, t921: F, t19905: F, t2155: F, t6064: F, t8123: F, t6155: F, t2531: F, t481: F, t6063: F) -> (F, F, F, F, F, F, F, F, F) {
    let t25555 = t7250 * t2219;
    let t25557 = t2670 * t6458;
    let t25560 = t19872 * t7926;
    let t25562 = t921 * t1543;
    let t25564 = t2155 * t19905 * t25562;
    let t25566 = t8123 * t6064;
    let t25567 = t6155 * t25566;
    let t25569 = t2531 * t481;
    let t25571 = t2155 * t6063 * t25569;
    (t25555, t25557, t25560, t25562, t25564, t25566, t25567, t25569, t25571)
}
