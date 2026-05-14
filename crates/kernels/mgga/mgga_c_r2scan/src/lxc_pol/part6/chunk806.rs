//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 806/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk806<F: Float>(t224: F, t5269: F, t5270: F, t1743: F, t720: F, t695: F, t4742: F, t4745: F, t4747: F, t4749: F, t4752: F, t5309: F, t5312: F, t5315: F) -> (F, F, F, F, F) {
    let t5538 = t224 * t5269;
    let t5539 = t5538 * t5270;
    let t5542 = t1743 * t720;
    let t5543 = t5542 * t695;
    let t5549 = -0.235315e1 * t5309 + 0.15687666666666666667e1 * t5312 - 0.7320911111111111111e1 * t5315 - t4742 + t4745 - t4747 - t4749 - t4752;
    (t5538, t5539, t5542, t5543, t5549)
}
