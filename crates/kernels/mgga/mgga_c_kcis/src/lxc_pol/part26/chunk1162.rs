//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1162/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1162<F: Float>(t1464: F, t1497: F, t58599: F, t7923: F, t1616: F, t7429: F, t1307: F, t22722: F, t6159: F, t1394: F, t20873: F, t27387: F, t102166: F, t102170: F, t12605: F, t1600: F, t21020: F, t27567: F, t27583: F, t27584: F, t27607: F, t28701: F, t28760: F, t29591: F, t29595: F, t4440: F, t77072: F, t7978: F, t7979: F, t99301: F) -> (F, F, F, F) {
    let t102205 = t1464 * t7923 * t58599 * t1497;
    let t102209 = t1616 * t7429;
    let t102221 = t6159 * t22722 * t1307;
    let t102237 = t1394 * t27387 * t20873;
    let t102239 = 0.51588271604938271604e-3 * t102205 + 0.46336805555555555556e-3 * t99301 * t28760 - 0.23168402777777777778e-3 * t27583 * t12605 * t102209 * t1307 + 0.23168402777777777778e-3 * t99301 * t28701 + 0.13901041666666666667e-2 * t27583 * t102166 + 0.13901041666666666667e-2 * t27583 * t102170 - 0.46377350260416666667e-4 * t27567 * t102221 + 0.11584201388888888889e-3 * t27583 * t4440 * t27584 * t77072 - 0.11584201388888888889e-3 * t27607 * t29591 - 0.11584201388888888889e-3 * t7978 * t1600 * t7979 * t21020 - 0.15445601851851851852e-3 * t27607 * t29595 - 0.11607361111111111111e-2 * t102237;
    (t102205, t102221, t102237, t102239)
}
