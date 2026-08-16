//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1025/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1025(t12703: f64, t374: f64, t1039: f64, t3696: f64, t2333: f64, t3347: f64, t1064: f64, t6897: f64, t3617: f64, t2332: f64, t6660: f64, t815: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12704 = t12703 * t374;
    let t12705 = t1039 * t3696;
    let t12706 = 2.0_f64 * t12705;
    let t13908 = t2333 * t3347;
    let t14160 = t6897 * t1064;
    let t14656 = t2333 * t3617;
    let t19025 = t2332 * t2332;
    let t19026 = 1.0_f64 / t19025;
    let t19146 = t815 * t6660;
    (t12704, t12706, t13908, t14160, t14656, t19026, t19146)
}
