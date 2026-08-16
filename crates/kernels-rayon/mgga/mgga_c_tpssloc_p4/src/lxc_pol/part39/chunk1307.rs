//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1307/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1307(t30315: f64, t510: f64, t1393: f64, t8273: f64, t2199: f64, t5107: f64, t1268: f64, t12725: f64, t2202: f64, t2314: f64, t26114: f64, t30266: f64, t30269: f64, t30272: f64, t30274: f64, t4028: f64, t4034: f64, t652: f64, t7458: f64, t7676: f64, t8190: f64, t8196: f64, t8260: f64, t8274: f64) -> (f64, f64, f64, f64) {
    let t30316 = t510 * t30315;
    let t30321 = t8273 * t1393;
    let t30326 = t5107 * t2199;
    let t30328 = t1268 * t30266 + t1268 * t30269 + t1268 * t30321 + t12725 * t2202 + t2202 * t26114 - t2314 * t8260 - t2314 * t8274 - t30272 * t652 - t30274 * t652 - t30316 * t652 - t30326 * t652 + t4028 * t8196 - t4034 * t8260 - t4034 * t8274 - t7458 * t8190 + t7676 * t8196;
    (t30316, t30321, t30326, t30328)
}
