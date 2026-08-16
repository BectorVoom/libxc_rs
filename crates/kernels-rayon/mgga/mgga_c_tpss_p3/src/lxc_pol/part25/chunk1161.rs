//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1161/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1161(t15220: f64, t16036: f64, t1168: f64, t118: f64, t1273: f64, t13133: f64, t1339: f64, t13554: f64, t13565: f64, t13974: f64, t14001: f64, t1604: f64, t1663: f64, t2056: f64, t3493: f64, t3502: f64, t3538: f64, t3542: f64, t4352: f64, t4541: f64, t4641: f64, t488: f64, t5322: f64, t544: f64, t5463: f64, t6103: f64, t646: f64) -> (f64, f64) {
    let t16037 = t15220 + t16036;
    let t16039 = t1168 * t5463 - t118 * t16037 + t1273 * t5322 - 4.0_f64 * t13133 * t1339 - 4.0_f64 * t1339 * t13554 - 2.0_f64 * t13565 * t646 + t13974 * t488 + t14001 * t544 + 2.0_f64 * t1604 * t4541 + 2.0_f64 * t1663 * t4352 - 4.0_f64 * t2056 * t4641 - 4.0_f64 * t3493 * t3502 - 4.0_f64 * t3493 * t3538 - 4.0_f64 * t3493 * t3542 - 4.0_f64 * t3538 * t6103;
    (t16037, t16039)
}
