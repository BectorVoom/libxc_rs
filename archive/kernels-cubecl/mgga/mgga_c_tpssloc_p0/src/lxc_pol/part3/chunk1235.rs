//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1235/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1235<F: Float>(t120: F, t5286: F, t3805: F, t3807: F, t12407: F, t5249: F, t12284: F, t12301: F, t12397: F, t12429: F, t1341: F, t1363: F, t16147: F, t16150: F, t16155: F, t16159: F, t16208: F, t16211: F, t16214: F, t16217: F, t16227: F, t16233: F, t16235: F, t16239: F, t16241: F, t1827: F, t3778: F, t3803: F, t5259: F, t5289: F) -> (F, F) {
    let t16242 = t120 * t5286;
    let t16244 = t3805 * t16242 * t3807;
    let t16248 = t3805 * t5249 * t12407;
    let t16253 = -t16147 + F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t1363 * t16150 + F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t1363 * t16155 + t16159 - t1341 * t16208 / F::cast_from(3072.0_f64) - F::cast_from(119.0_f64) / F::cast_from(13824.0_f64) * t16211 + t16214 - F::cast_from(5.0_f64) / F::cast_from(128.0_f64) * t1363 * t16217 - t12397 * t1827 / F::cast_from(3072.0_f64) - t3778 * t5289 / F::cast_from(1536.0_f64) - F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t3803 * t16227 - F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t12284 + F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t12301 - t16233 * t16235 / F::cast_from(512.0_f64) - t16239 + t16241 + t3803 * t16244 / F::cast_from(384.0_f64) + t3803 * t16248 / F::cast_from(768.0_f64) + t12429 * t5259 / F::cast_from(384.0_f64);
    (t16242, t16253)
}
