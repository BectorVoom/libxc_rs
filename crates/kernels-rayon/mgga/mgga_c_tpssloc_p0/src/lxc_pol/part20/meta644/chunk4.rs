//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2362/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2362(t14234: f64, t3070: f64, t42488: f64, t10390: f64, t10408: f64, t10413: f64, t10445: f64, t1046: f64, t13527: f64, t14218: f64, t14219: f64, t14228: f64, t14230: f64, t1611: f64, t2244: f64, t2250: f64, t2770: f64, t3071: f64, t360: f64, t369: f64, t378: f64, t42303: f64, t48428: f64, t48431: f64, t48432: f64, t48441: f64, t48446: f64, t48460: f64, t68: f64) -> f64 {
    let t48463 = t3070 * t42488 * t14234;
    let t48471 = t48431 + t48432 * t1046 / 1536.0_f64 + 19.0_f64 / 1296.0_f64 * t42303 + t48428 * t68 * t369 * t378 / 3072.0_f64 - t48441 / 36.0_f64 - 209.0_f64 / 2592.0_f64 * t1611 * t10445 * t378 + 19.0_f64 / 864.0_f64 * t48446 - t10413 * t3071 * t14218 * t14219 * t2250 / 1536.0_f64 - 5.0_f64 / 4608.0_f64 * t10413 * t10408 * t14218 * t360 * t2770 * t2244 - t48460 / 576.0_f64 + 5.0_f64 / 3456.0_f64 * t48463 - t10390 * t14230 / 384.0_f64 + 5.0_f64 / 2304.0_f64 * t3070 * t10408 * t13527 * t14228;
    t48471
}
