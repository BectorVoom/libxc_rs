//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1063/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1063<F: Float>(t10626: F, t12056: F, t3275: F, t11458: F, t40282: F, t38715: F, t40394: F, t11455: F, t11336: F, t40594: F, t40595: F, t1115: F, t39190: F, t39192: F, t1146: F, t2449: F, t2881: F, t3560: F, t3570: F, t41240: F, t41243: F, t41247: F, t41251: F, t41254: F, t41256: F, t41258: F, t8306: F) -> (F, F, F, F, F, F, F) {
    let t41261 = t3275 * t12056 * t10626 / 2.0;
    let t41263 = 3.0 / 2.0 * t40282 * t11458;
    let t41265 = 3.0 / 2.0 * t40394 * t38715;
    let t41270 = 15.0 / 8.0 * t40282 * t11455;
    let t41273 = 45.0 / 32.0 * t40594 * t11336 * t40595;
    let t41276 = 135.0 / 32.0 * t39190 * t1115 * t39192;
    let t41277 = t1146 * t8306 + 2.0 * t2449 * t3570 + 2.0 * t2881 * t3560 + t41240 - t41243 - t41247 + t41251 + t41254 + t41256 + t41258 + t41261 - t41263 + t41265 - t41270 - t41273 + t41276;
    (t41261, t41263, t41265, t41270, t41273, t41276, t41277)
}
