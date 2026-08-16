//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1213/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1213(t3275: f64, t3465: f64, t42384: f64, t42403: f64, t11345: f64, t12422: f64, t11523: f64, t12203: f64, t11625: f64, t12056: f64, t3472: f64, t42428: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44108 = 3.0_f64 / 2.0_f64 * t3275 * t3465 * t42384;
    let t44110 = t3275 * t3465 * t42403;
    let t44113 = t12422 * t11345 / 4.0_f64;
    let t44115 = 5.0_f64 / 8.0_f64 * t11523 * t12203;
    let t44117 = t3275 * t12056 * t11625;
    let t44120 = 5.0_f64 / 16.0_f64 * t3275 * t3472 * t42428;
    (t44108, t44110, t44113, t44115, t44117, t44120)
}
