//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 702/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk702(t5290: f64, t76: f64, t1982: f64, t5270: f64, t1917: f64, t695: f64, t5266: f64, t224: f64, t5269: f64, t1743: f64, t720: f64, t4742: f64, t4745: f64, t4747: f64, t4749: f64, t4752: f64, t5309: f64, t5312: f64, t5315: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5530 = t76 * t5290;
    let t5531 = t1982 * t5270;
    let t5534 = t1917 * t695;
    let t5537 = t76 * t5266;
    let t5538 = t224 * t5269;
    let t5539 = t5538 * t5270;
    let t5542 = t1743 * t720;
    let t5543 = t5542 * t695;
    let t5549 = -0.235315e1_f64 * t5309 + 0.15687666666666666667e1_f64 * t5312 - 0.7320911111111111111e1_f64 * t5315 - t4742 + t4745 - t4747 - t4749 - t4752;
    (t5530, t5531, t5534, t5537, t5539, t5543, t5549)
}
