//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1061/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1061(t122: f64, t607: f64, t10928: f64, t3434: f64, t874: f64, t1266: f64, t550: f64, t1104: f64, t3429: f64, t58: f64, t875: f64, t3446: f64, t766: f64) -> (f64, f64, f64, f64, f64) {
    let t37465 = t607 * t122;
    let t37468 = t3434 * t10928 * t37465 * t874;
    let t37470 = t550 * t1266;
    let t37472 = t3429 * t37470 * t1104;
    let t37475 = t1266 * t875 * t58;
    let t37477 = t3446 * t37475 * t766;
    (t37468, t37470, t37472, t37475, t37477)
}
