//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1074/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1074<F: Float>(t1409: F, t1506: F, t234: F, t4816: F, t4998: F, t732: F, t4994: F, t1497: F, t454: F, t4854: F, t1414: F, t23: F, t122: F, t306: F, t307: F, t296: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19061 = 0.62337092780453269531e3 * t234 * t4816 * t1409 * t1506;
    let t19062 = t732 * t4998;
    let t19064 = t732 * t4994;
    let t19069 = 0.46785788981077169656e1 * t234 * t1497 * t4854 * t454;
    let t19091 = 1.0 / t23 / t1414;
    let t19092 = t122 * t19091;
    let t19093 = 2618.0 / 81.0 * t19092;
    let t19105 = t306 * t306;
    let t19107 = 1.0 / t307 / t19105;
    let t19129 = t296 * t296;
    (t19061, t19062, t19064, t19069, t19091, t19092, t19093, t19107, t19129)
}
