//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1306/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1306<F: Float>(t110605: F, t2075: F, t33437: F, t26823: F, t5600: F, t9461: F, t1339: F, t26790: F, t110435: F, t1327: F, t6204: F, t8063: F, t1322: F, t7706: F, t110379: F, t6175: F) -> (F, F, F, F, F, F, F) {
    let t118741 = t110605 * t2075 * t33437;
    let t118745 = t5600 * t9461 * t26823;
    let t118748 = t1339 * t9461 * t26790;
    let t118754 = t6204 * t110435 * t8063 * t1327;
    let t118759 = t7706 * t1322;
    let t118764 = t7706 * t1327;
    let t118766 = t6175 * t110379 * t118764;
    (t118741, t118745, t118748, t118754, t118759, t118764, t118766)
}
