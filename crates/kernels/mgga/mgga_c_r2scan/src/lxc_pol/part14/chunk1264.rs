//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1264/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1264<F: Float>(t3269: F, t42263: F, t3579: F, t38775: F, t12051: F, t1551: F, t40603: F, t38211: F, t38216: F, t38220: F, t39106: F, t39107: F, t39108: F, t40587: F, t42253: F, t42255: F, t42257: F, t42260: F) -> (F, F, F, F) {
    let t42265 = t3269 * t42263 / F::cast_from(2.0_f64);
    let t42267 = t3579 * t38775 / F::cast_from(4.0_f64);
    let t42270 = t3579 * t1551 * t12051 / F::cast_from(4.0_f64);
    let t42274 = F::cast_from(0.3842256877732895568e-2_f64) * t40603;
    let t42275 = -t42253 + t42255 - t42257 - t42260 - F::cast_from(0.16163010989689081288e-5_f64) * t40587 + t42265 + t42267 - t42270 + F::cast_from(0.12195059916630011325e-2_f64) * t38211 - F::cast_from(0.30487649791575028312e-3_f64) * t38216 + F::cast_from(0.43368970657079495308e-4_f64) * t38220 - t39106 - t39107 + t39108 + t42274;
    (t42265, t42267, t42270, t42275)
}
