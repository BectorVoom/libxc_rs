//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1024/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1024(t12365: f64, t374: f64, t1039: f64, t3570: f64, t1149: f64, t2449: f64, t2333: f64, t3492: f64, t3718: f64, t2332: f64, t1269: f64, t1275: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12366 = t12365 * t374;
    let t12367 = t1039 * t3570;
    let t12368 = t2449 * t1149;
    let t14402 = t2333 * t3492;
    let t15059 = t2333 * t3718;
    let t19025 = t2332 * t2332;
    let t19026 = 1.0_f64 / t19025;
    let t19141 = t1269 * t1275;
    (t12366, t12367, t12368, t14402, t15059, t19026, t19141)
}
