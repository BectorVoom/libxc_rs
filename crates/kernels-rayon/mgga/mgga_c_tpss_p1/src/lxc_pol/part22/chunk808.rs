//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 808/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk808(t4523: f64, t4540: f64, t1163: f64, t1168: f64, t118: f64, t1273: f64, t1322: f64, t1339: f64, t1600: f64, t1604: f64, t1663: f64, t2056: f64, t3491: f64, t3493: f64, t3499: f64, t3502: f64, t3538: f64, t3542: f64, t4341: f64, t4352: f64, t485: f64, t488: f64, t544: f64, t624: f64, t626: f64, t646: f64) -> (f64, f64) {
    let t4541 = t4523 + t4540;
    let t4543 = -t1163 * t1322 + t1168 * t1663 - t118 * t4341 + t1273 * t1604 - 2.0_f64 * t1339 * t2056 - 2.0_f64 * t1339 * t3499 - t1600 * t624 - t3491 * t485 - 2.0_f64 * t3493 * t646 - 2.0_f64 * t3502 * t626 - 2.0_f64 * t3538 * t626 - 2.0_f64 * t3542 * t626 + t4352 * t544 + t4541 * t488;
    (t4541, t4543)
}
