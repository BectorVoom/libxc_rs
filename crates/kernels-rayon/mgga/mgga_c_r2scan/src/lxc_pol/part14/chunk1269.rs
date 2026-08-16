//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1269/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1269(t39030: f64, t40630: f64, t40631: f64, t12197: f64, t1561: f64, t3275: f64, t3277: f64, t10630: f64, t12056: f64, t3262: f64, t3352: f64, t41202: f64) -> (f64, f64, f64, f64) {
    let t42330 = 3.0_f64 * t40630 * t39030 * t40631;
    let t42331 = t1561 * t12197;
    let t42334 = 5.0_f64 / 8.0_f64 * t3275 * t42331 * t3277;
    let t42339 = 3.0_f64 / 4.0_f64 * t3262 * t12056 * t10630;
    let t42344 = t3275 * t41202 * t3352 / 2.0_f64;
    (t42330, t42334, t42339, t42344)
}
