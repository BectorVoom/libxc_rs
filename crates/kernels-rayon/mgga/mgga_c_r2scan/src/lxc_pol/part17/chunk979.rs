//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 979/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk979(t11888: f64, t1276: f64, t3675: f64, t856: f64, t11189: f64, t11621: f64, t3275: f64, t11465: f64, t3579: f64, t11555: f64, t3472: f64, t11336: f64, t3270: f64, t986: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11889 = t1276 * t11888;
    let t11993 = t3675 * t856;
    let t12024 = t11189 * t11621;
    let t12025 = t3275 * t12024;
    let t12026 = 45.0_f64 / 64.0_f64 * t12025;
    let t12027 = t3579 * t11465;
    let t12028 = 5.0_f64 / 16.0_f64 * t12027;
    let t12029 = t3472 * t11555;
    let t12030 = t3275 * t12029;
    let t12031 = 5.0_f64 / 16.0_f64 * t12030;
    let t12033 = t3270 * t11336 * t986;
    (t11889, t11993, t12024, t12025, t12026, t12027, t12028, t12029, t12030, t12031, t12033)
}
