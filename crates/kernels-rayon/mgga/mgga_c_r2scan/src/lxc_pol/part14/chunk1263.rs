//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1263/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1263(t12024: f64, t37271: f64, t12045: f64, t37282: f64, t11342: f64, t40681: f64, t12051: f64, t1554: f64, t3579: f64, t15059: f64, t795: f64, t3270: f64) -> (f64, f64, f64, f64, f64) {
    let t42253 = 45.0_f64 / 32.0_f64 * t37271 * t12024;
    let t42255 = 3.0_f64 / 2.0_f64 * t37282 * t12045;
    let t42257 = 3.0_f64 / 2.0_f64 * t40681 * t11342;
    let t42260 = t3579 * t1554 * t12051 / 4.0_f64;
    let t42262 = t15059 * t795;
    let t42263 = t3270 * t42262;
    (t42253, t42255, t42257, t42260, t42263)
}
