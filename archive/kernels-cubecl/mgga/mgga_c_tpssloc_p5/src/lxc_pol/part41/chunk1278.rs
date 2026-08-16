//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1278/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1278<F: Float>(t30315: F, t510: F, t1393: F, t8273: F, t2199: F, t5107: F, t1268: F, t12725: F, t2202: F, t2314: F, t26114: F, t30266: F, t30269: F, t30272: F, t30274: F, t4028: F, t4034: F, t652: F, t7458: F, t7676: F, t8190: F, t8196: F, t8260: F, t8274: F) -> (F, F, F, F) {
    let t30316 = t510 * t30315;
    let t30321 = t8273 * t1393;
    let t30326 = t5107 * t2199;
    let t30328 = t1268 * t30266 + t1268 * t30269 + t1268 * t30321 + t12725 * t2202 + t2202 * t26114 - t2314 * t8260 - t2314 * t8274 - t30272 * t652 - t30274 * t652 - t30316 * t652 - t30326 * t652 + t4028 * t8196 - t4034 * t8260 - t4034 * t8274 - t7458 * t8190 + t7676 * t8196;
    (t30316, t30321, t30326, t30328)
}
