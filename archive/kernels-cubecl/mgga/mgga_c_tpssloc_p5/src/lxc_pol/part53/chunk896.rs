//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 896/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk896<F: Float>(t5: F, t32244: F, t9239: F, t33: F, t8705: F, t2240: F, t20: F, t60: F, t131: F, t8308: F, t8302: F, t31000: F, t31006: F, t31013: F, t31024: F, t8707: F) -> (F, F, F, F, F, F, F, F) {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t32245 = t9239 * t32244;
    let t32248 = t33 * t8705;
    let t32249 = t2240 * t32248;
    let t32253 = F::cast_from(1.0_f64) / t60 / t20;
    let t32255 = t32253 * t131 * t8308;
    let t32257 = F::cast_from(20.0_f64) / F::cast_from(27.0_f64) * t8302 * t32255;
    let t32258 = t2240 * t32244;
    let t32262 = piecewise3::<F>(t8, F::cast_from(0.0_f64), F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t31000 * t8707 - F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t32245 * t31006 - F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t32249 * t31013 - t32257 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t32258 * t31024);
    (t32245, t32248, t32249, t32253, t32255, t32257, t32258, t32262)
}
