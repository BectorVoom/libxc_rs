//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1114/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1114<F: Float>(t11282: F, t6068: F, t11285: F, t1155: F, t1164: F, t11292: F, t4883: F, t15218: F, t4882: F, t1190: F, t6238: F, t1743: F, t4965: F) -> (F, F, F, F, F) {
    let t18274 = t11282 * t6068;
    let t18275 = t11285 * t1155;
    let t18276 = t18274 * t18275;
    let t18278 = F::cast_from(0.10254018858216406658e4_f64) * t1164 * t18276;
    let t18279 = t11292 * t6068;
    let t18280 = t18279 * t4883;
    let t18282 = F::cast_from(0.10389515463408878255e3_f64) * t1164 * t18280;
    let t18283 = t4882 * t15218;
    let t18285 = F::cast_from(0.34631718211362927518e2_f64) * t1164 * t18283;
    let t18287 = t1190 * t6238;
    let t18297 = t4965 * t1743;
    (t18278, t18282, t18285, t18287, t18297)
}
