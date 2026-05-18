//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1196/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1196<F: Float>(t11336: F, t40594: F, t40595: F, t1115: F, t39190: F, t39192: F, t1146: F, t2449: F, t2881: F, t3560: F, t3570: F, t41240: F, t41243: F, t41247: F, t41251: F, t41254: F, t41256: F, t41258: F, t41261: F, t41263: F, t41265: F, t41270: F, t8306: F) -> (F, F, F) {
    let t41273 = F::new(45.0) / F::new(32.0) * t40594 * t11336 * t40595;
    let t41276 = F::new(135.0) / F::new(32.0) * t39190 * t1115 * t39192;
    let t41277 = t1146 * t8306 + F::new(2.0) * t2449 * t3570 + F::new(2.0) * t2881 * t3560 + t41240 - t41243 - t41247 + t41251 + t41254 + t41256 + t41258 + t41261 - t41263 + t41265 - t41270 - t41273 + t41276;
    (t41273, t41276, t41277)
}
