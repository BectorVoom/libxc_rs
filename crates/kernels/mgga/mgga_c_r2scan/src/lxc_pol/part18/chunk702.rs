//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 702/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk702<F: Float>(t5290: F, t76: F, t1982: F, t5270: F, t1917: F, t695: F, t5266: F, t224: F, t5269: F, t1743: F, t720: F, t4742: F, t4745: F, t4747: F, t4749: F, t4752: F, t5309: F, t5312: F, t5315: F) -> (F, F, F, F, F, F, F) {
    let t5530 = t76 * t5290;
    let t5531 = t1982 * t5270;
    let t5534 = t1917 * t695;
    let t5537 = t76 * t5266;
    let t5538 = t224 * t5269;
    let t5539 = t5538 * t5270;
    let t5542 = t1743 * t720;
    let t5543 = t5542 * t695;
    let t5549 = -F::new(0.235315e1) * t5309 + F::cast_from(0.15687666666666666667e1_f64) * t5312 - F::cast_from(0.7320911111111111111e1_f64) * t5315 - t4742 + t4745 - t4747 - t4749 - t4752;
    (t5530, t5531, t5534, t5537, t5539, t5543, t5549)
}
