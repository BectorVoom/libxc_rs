//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1259/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1259(t3579: f64, t39274: f64, t31498: f64, t3263: f64, t3275: f64, t2867: f64, t40324: f64, t11622: f64, t40713: f64, t12396: f64, t37282: f64, t2847: f64, t3582: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43968 = t3579 * t39274 / 2.0_f64;
    let t43971 = t3275 * t3263 * t31498 / 4.0_f64;
    let t43974 = t3275 * t40324 * t2867 / 2.0_f64;
    let t43976 = 45.0_f64 / 32.0_f64 * t40713 * t11622;
    let t43978 = 15.0_f64 / 8.0_f64 * t37282 * t12396;
    let t43979 = t3582 * t2847;
    (t43968, t43971, t43974, t43976, t43978, t43979)
}
