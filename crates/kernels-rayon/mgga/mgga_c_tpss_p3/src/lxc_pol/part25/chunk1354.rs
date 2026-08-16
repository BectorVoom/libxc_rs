//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1354/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1354(t1270: f64, t13133: f64, t1339: f64, t13554: f64, t16037: f64, t1760: f64, t1799: f64, t18547: f64, t19577: f64, t19579: f64, t19604: f64, t19609: f64, t20226: f64, t20227: f64, t20289: f64, t20357: f64, t20374: f64, t20386: f64, t2056: f64, t21855: f64, t21883: f64, t21894: f64, t21900: f64, t25469: f64, t3493: f64, t3499: f64, t3502: f64, t509: f64, t544: f64, t5706: f64, t5757: f64, t6103: f64, t61801: f64, t6243: f64, t626: f64, t6324: f64, t6413: f64, t68798: f64, t71344: f64, t71574: f64, t71603: f64, t71662: f64, t71715: f64, t71823: f64, t71872: f64) -> f64 {
    let t71878 = -4.0_f64 * t71344 * t1339 - 4.0_f64 * t20289 * t3502 + 6.0_f64 * t6243 * t20227 - 6.0_f64 * t18547 * t25469 * t19609 - 6.0_f64 * t61801 * t21900 + 4.0_f64 * t19579 * t20357 * t68798 - 4.0_f64 * t6103 * t20374 - 2.0_f64 * t2056 * t21894 - 2.0_f64 * t3499 * t21894 - 2.0_f64 * t626 * t16037 * t1799 - 4.0_f64 * t13133 * t6324 - 4.0_f64 * t13554 * t6324 - 4.0_f64 * t3493 * t20386 + (t71574 + t71603) * t544 + 6.0_f64 * t19577 * t6413 - t1760 * t21855 * t5757 + 6.0_f64 * t1760 * t20226 * t19604 + 3.0_f64 * t5706 * t21883 + t1760 * t509 * (t71662 + t71715 + t71823 + t71872) * t1270;
    t71878
}
