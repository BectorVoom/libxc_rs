//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1032/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1032(t12964: f64, t354: f64, t1146: f64, t3250: f64, t2333: f64, t3492: f64, t3718: f64, t2332: f64, t6660: f64, t815: f64, t312: f64, t320: f64, t6659: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12965 = t354 * t12964;
    let t12966 = t1146 * t3250;
    let t14402 = t2333 * t3492;
    let t15059 = t2333 * t3718;
    let t19025 = t2332 * t2332;
    let t19026 = 1.0_f64 / t19025;
    let t19146 = t815 * t6660;
    let t19155 = t312 / t6659 / t320;
    (t12965, t12966, t14402, t15059, t19026, t19146, t19155)
}
