//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1264/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1264(t3269: f64, t42263: f64, t3579: f64, t38775: f64, t12051: f64, t1551: f64, t40603: f64, t38211: f64, t38216: f64, t38220: f64, t39106: f64, t39107: f64, t39108: f64, t40587: f64, t42253: f64, t42255: f64, t42257: f64, t42260: f64) -> (f64, f64, f64, f64) {
    let t42265 = t3269 * t42263 / 2.0_f64;
    let t42267 = t3579 * t38775 / 4.0_f64;
    let t42270 = t3579 * t1551 * t12051 / 4.0_f64;
    let t42274 = 0.3842256877732895568e-2_f64 * t40603;
    let t42275 = -t42253 + t42255 - t42257 - t42260 - 0.16163010989689081288e-5_f64 * t40587 + t42265 + t42267 - t42270 + 0.12195059916630011325e-2_f64 * t38211 - 0.30487649791575028312e-3_f64 * t38216 + 0.43368970657079495308e-4_f64 * t38220 - t39106 - t39107 + t39108 + t42274;
    (t42265, t42267, t42270, t42275)
}
