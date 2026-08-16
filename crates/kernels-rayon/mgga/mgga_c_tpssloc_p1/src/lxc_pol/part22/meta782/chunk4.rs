//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2676/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2676(t1358: f64, t20596: f64, t12283: f64, t20442: f64, t120: f64, t20356: f64, t20465: f64, t1351: f64, t40046: f64, t12429: f64, t1352: f64, t16224: f64, t16233: f64, t16305: f64, t16306: f64, t16394: f64, t1825: f64, t19744: f64, t19876: f64, t19945: f64, t19976: f64, t19994: f64, t20004: f64, t20450: f64, t20463: f64, t3803: f64, t40168: f64, t5246: f64, t5248: f64, t5308: f64, t54048: f64, t54744: f64, t6388: f64, t74120: f64) -> (f64, f64) {
    let t74578 = t20596 * t1358;
    let t74584 = t12283 * t20442;
    let t74592 = t120 * t20356;
    let t74597 = t12283 * t20465;
    let t74599 = t40046 * t1351;
    let t74610 = -5.0_f64 / 256.0_f64 * t3803 * t16224 * t1825 * t19994 + t3803 * t16305 * t16306 * t20463 / 256.0_f64 - 7.0_f64 / 4608.0_f64 * t74578 - t54048 + 5.0_f64 / 128.0_f64 * t5246 * t16224 * t6388 * t5308 + 7.0_f64 / 1536.0_f64 * t74584 - t16394 * t19976 / 1024.0_f64 - t19876 * t20004 / 128.0_f64 + t19876 * t19945 / 256.0_f64 + 5.0_f64 / 128.0_f64 * t3803 * t40168 * t74592 * t1352 - 7.0_f64 / 384.0_f64 * t74597 + t54744 * t5248 * t74120 * t74599 / 128.0_f64 - 3.0_f64 / 256.0_f64 * t16233 * t5248 * t74120 * t19744 - 5.0_f64 / 256.0_f64 * t12429 * t20450;
    (t74599, t74610)
}
