//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1377/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1377(t13133: f64, t1338: f64, t13473: f64, t13554: f64, t1600: f64, t1800: f64, t18547: f64, t20319: f64, t20357: f64, t20371: f64, t20374: f64, t20396: f64, t2056: f64, t20640: f64, t20642: f64, t21011: f64, t21180: f64, t21236: f64, t21750: f64, t21880: f64, t24128: f64, t3493: f64, t3499: f64, t4541: f64, t5801: f64, t5809: f64, t5816: f64, t6243: f64, t626: f64, t6318: f64, t6409: f64, t645: f64, t68958: f64, t69069: f64, t69383: f64) -> f64 {
    let t72682 = -4.0_f64 * t21180 * t5816 - 4.0_f64 * t13133 * t6318 - 4.0_f64 * t13554 * t6318 - 4.0_f64 * t3493 * t20396 - 4.0_f64 * t3493 * t20374 - 2.0_f64 * t69069 * t1800 - 2.0_f64 * t69383 * t1800 - 2.0_f64 * t21236 * t5809 - 2.0_f64 * t6243 * t20642 - 6.0_f64 * t18547 * t24128 * t21011 + 6.0_f64 * t18547 * t20357 * t68958 - 2.0_f64 * t626 * t21750 * t645 - 4.0_f64 * t626 * t1600 * t20319 - 4.0_f64 * t2056 * t21880 - 4.0_f64 * t3499 * t21880 - 4.0_f64 * t626 * t20640 * t1338 - 4.0_f64 * t5801 * t13473 - 4.0_f64 * t3493 * t20371 + 2.0_f64 * t6409 * t4541;
    t72682
}
