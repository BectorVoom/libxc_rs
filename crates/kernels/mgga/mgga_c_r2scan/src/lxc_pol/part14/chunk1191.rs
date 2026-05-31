//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1191/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1191<F: Float>(t11199: F, t3275: F, t7040: F, t3579: F, t38678: F, t11189: F, t40464: F, t3262: F, t3472: F, t40416: F, t11020: F, t12086: F) -> (F, F, F, F, F) {
    let t41211 = t3275 * t11199 * t7040 / F::cast_from(2.0_f64);
    let t41213 = t3579 * t38678 / F::cast_from(4.0_f64);
    let t41216 = F::cast_from(45.0_f64) / F::cast_from(64.0_f64) * t3275 * t11189 * t40464;
    let t41219 = F::cast_from(15.0_f64) / F::cast_from(8.0_f64) * t3262 * t3472 * t40416;
    let t41221 = t11020 * t12086 / F::cast_from(4.0_f64);
    (t41211, t41213, t41216, t41219, t41221)
}
