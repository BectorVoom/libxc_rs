//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1262/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1262<F: Float>(t12198: F, t3270: F, t3269: F, t3275: F, t3465: F, t39335: F, t11338: F, t11523: F, t39030: F, t40574: F, t40575: F, t37556: F, t37564: F, t37580: F, t39097: F, t39099: F, t40556: F, t40559: F, t40564: F, t42143: F, t42146: F, t42148: F) -> (F, F, F, F, F) {
    let t42234 = t3270 * t12198;
    let t42236 = t3269 * t42234 / F::cast_from(2.0_f64);
    let t42239 = t3275 * t3465 * t39335 / F::cast_from(2.0_f64);
    let t42244 = t11523 * t11338 / F::cast_from(2.0_f64);
    let t42248 = F::cast_from(5.0_f64) / F::cast_from(4.0_f64) * t40574 * t39030 * t40575;
    let t42249 = t42143 + F::cast_from(0.325201597776800302e-2_f64) * t37556 + t39097 - F::cast_from(0.60975299583150056624e-3_f64) * t37564 - t39099 - t42146 + t42148 - t42236 - t42239 + F::cast_from(0.162600798888400151e-2_f64) * t40556 + F::cast_from(0.3842256877732895568e-2_f64) * t40559 - F::cast_from(0.86737941314158990616e-4_f64) * t40564 + t42244 + F::cast_from(0.13680077012009379e-5_f64) * t37580 + t42248;
    (t42236, t42239, t42244, t42248, t42249)
}
