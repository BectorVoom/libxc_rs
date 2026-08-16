//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 793/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk793(t44: f64, t1361: f64, t35: f64, t1216: f64, t415: f64, t1213: f64, t1219: f64, t2466: f64, t2469: f64, t40: f64, t48: f64, t6976: f64, t4948: f64, t893: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t45 = t44 <= zeta_threshold;
    let t6979 = t1361 * t35;
    let t6980 = t1216 * t415;
    let t6990 = piecewise3(t45, 0.0_f64, -8.0_f64 / 27.0_f64 * t6976 * t1213 + 16.0_f64 / 9.0_f64 * t6979 * t6980 + 4.0_f64 / 9.0_f64 * t2466 * t1219 + 8.0_f64 / 3.0_f64 * t48 * t1216 - 8.0_f64 * t2469 * t40);
    let t6991 = t4948 * t893;
    (t6980, t6990, t6991)
}
