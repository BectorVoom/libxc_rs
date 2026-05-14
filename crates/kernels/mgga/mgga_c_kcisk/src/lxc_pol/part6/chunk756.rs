//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 756/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk756<F: Float>(t1873: F, t28290: F, t1869: F, t2527: F, t8786: F, t1899: F, t23033: F, t2364: F, t1800: F, t1799: F, t8780: F, t5203: F, t6719: F, t8882: F, t10447: F, t967: F) -> (F, F, F, F, F, F, F, F) {
    let t28291 = t1873 * t28290;
    let t28292 = t1869 * t28291;
    let t28294 = t8786 * t2527;
    let t28295 = t1899 * t28294;
    let t28296 = t1873 * t28295;
    let t28297 = t1869 * t28296;
    let t28299 = t23033 * t2364;
    let t28300 = t1800 * t28299;
    let t28301 = t1799 * t28300;
    let t28303 = t8780 * t2527;
    let t28304 = t5203 * t28303;
    let t28305 = t1873 * t28304;
    let t28306 = t1869 * t28305;
    let t28308 = t6719 * t8882;
    let t28309 = t1869 * t28308;
    let t28312 = 6.0 * t967 + 6.0 * t10447;
    (t28292, t28294, t28297, t28301, t28303, t28306, t28309, t28312)
}
