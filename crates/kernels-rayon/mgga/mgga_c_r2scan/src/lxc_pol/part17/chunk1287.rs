//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1287/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1287(t11199: f64, t3275: f64, t8601: f64, t12029: f64, t40713: f64, t12042: f64, t38339: f64, t38356: f64, t38359: f64, t39122: f64, t39127: f64, t39129: f64, t39130: f64, t39131: f64, t39134: f64, t45083: f64, t45085: f64, t45088: f64, t45094: f64) -> (f64, f64, f64, f64) {
    let t45097 = t3275 * t11199 * t8601 / 4.0_f64;
    let t45099 = 5.0_f64 / 8.0_f64 * t40713 * t12029;
    let t45100 = t40713 * t12042;
    let t45101 = -t45083 - t45085 - t45088 + t39122 + t39127 + 0.162600798888400151e-2_f64 * t38339 - t39129 + t39130 - t39131 - 0.38422568777328955681e-2_f64 * t38356 + 0.60975299583150056624e-3_f64 * t38359 + t39134 + t45094 + t45097 - t45099 - t45100;
    (t45097, t45099, t45100, t45101)
}
