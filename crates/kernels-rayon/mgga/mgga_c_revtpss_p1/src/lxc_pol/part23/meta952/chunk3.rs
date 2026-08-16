//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3158/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3158(t1042: f64, t1261: f64, t12787: f64, t17569: f64, t17736: f64, t20792: f64, t20800: f64, t20811: f64, t20950: f64, t21143: f64, t3362: f64, t3720: f64, t4181: f64, t5302: f64, t5304: f64, t5340: f64, t5381: f64, t57056: f64, t6573: f64, t6631: f64, t69971: f64, t69984: f64, t70006: f64, t70008: f64, t78770: f64) -> f64 {
    let t82978 = 0.15244095330869239812e-2_f64 * t69971 + 0.95275595817932748826e-3_f64 * t69984 + 0.7145669686344956162e-3_f64 * t21143 * t5304 + 0.7145669686344956162e-3_f64 * t5381 * t20792 + 0.23818898954483187207e-3_f64 * t1261 * t1042 * t5302 * t78770 - 0.28582678745379824648e-3_f64 * t70006 + 0.30488190661738479624e-2_f64 * t70008 + 0.14291339372689912324e-2_f64 * t17736 * t12787 * t6573 * t3362 * t4181 + 0.12862205435420921092e-2_f64 * t5340 * t3720 * t20800 * t20950 - 0.68598428988911579154e-2_f64 * t57056 * t6631 + 0.42874018118069736972e-3_f64 * t17569 * t20811;
    t82978
}
