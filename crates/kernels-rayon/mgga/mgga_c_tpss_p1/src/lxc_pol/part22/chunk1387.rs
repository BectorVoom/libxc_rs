//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1387/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1387(t1163: f64, t13131: f64, t13133: f64, t13136: f64, t13225: f64, t1339: f64, t1830: f64, t1834: f64, t18544: f64, t18547: f64, t18690: f64, t18711: f64, t18717: f64, t18898: f64, t18930: f64, t19579: f64, t20137: f64, t20319: f64, t20357: f64, t20374: f64, t2056: f64, t20642: f64, t3493: f64, t3499: f64, t3538: f64, t41839: f64, t43998: f64, t4541: f64, t485: f64, t5706: f64, t5801: f64, t5820: f64, t5905: f64, t6243: f64, t626: f64, t6437: f64, t67538: f64, t67552: f64) -> f64 {
    let t67715 = 4.0_f64 * t19579 * t20357 * t43998 - 4.0_f64 * t2056 * t20374 - 4.0_f64 * t3499 * t20374 - 4.0_f64 * t626 * t1163 * t20319 - 4.0_f64 * t18898 * t3538 - 2.0_f64 * t67552 * t1339 + 6.0_f64 * t6243 * t18711 + 3.0_f64 * t6243 * t18717 + 6.0_f64 * t5706 * t20137 + 2.0_f64 * t5905 * t4541 + t1834 * t13131 - 2.0_f64 * t13136 * t1830 - 2.0_f64 * t626 * t485 * t67538 - 4.0_f64 * t13133 * t5820 - 4.0_f64 * t3493 * t18930 + t18544 * t6437 - 3.0_f64 * t18547 * t18690 * t41839 - 2.0_f64 * t5706 * t20642 - 4.0_f64 * t5801 * t13225;
    t67715
}
