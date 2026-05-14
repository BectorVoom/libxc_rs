//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1134/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1134<F: Float>(t5686: F, t652: F, t1663: F, t390: F, t1890: F, t5763: F, t644: F, t188: F, t5448: F, t5671: F, t5674: F, t5403: F, t5408: F, t1800: F, t1883: F, t632: F) -> (F, F, F, F, F, F) {
    let t21358 = t652 * t5686;
    let t21361 = 0.11455730062901982479e1 * t390 * t1663 * t21358;
    let t21365 = 0.11053848960848725644e3 * t390 * t1890 * t644 * t5763;
    let t21370 = 0.17776777237001298852e4 * t390 * t5671 * t188 * t5674 * t5448;
    let t21371 = t5408 * t5403;
    let t21375 = 12.0 * t632 * t1883 * t1800;
    (t21358, t21361, t21365, t21370, t21371, t21375)
}
