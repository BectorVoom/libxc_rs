//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1391/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1391(t116: f64, t20319: f64, t117: f64, t1279: f64, t1281: f64, t13220: f64, t13265: f64, t1668: f64, t1853: f64, t19041: f64, t19044: f64, t19047: f64, t2061: f64, t20660: f64, t20682: f64, t20685: f64, t20690: f64, t20691: f64, t2105: f64, t3403: f64, t3410: f64, t4549: f64, t4556: f64, t547: f64, t5947: f64, t5953: f64, t5954: f64, t5957: f64, t6323: f64, t6446: f64, t645: f64, t6452: f64, t67538: f64) -> f64 {
    let t67816 = t116 * t20319;
    let t67843 = 3.0_f64 * t547 * t117 * t67538 + 12.0_f64 * t1279 * t20682 + 6.0_f64 * t4549 * t5957 + 3.0_f64 * t6446 * t3410 + 6.0_f64 * t3403 * t6452 + 12.0_f64 * t1279 * t20685 + 12.0_f64 * t1668 * t19041 + 12.0_f64 * t547 * t67816 * t645 + 12.0_f64 * t4549 * t5954 + 6.0_f64 * t20660 * t1281 + 3.0_f64 * t13265 * t1853 + 12.0_f64 * t5947 * t4556 + 6.0_f64 * t547 * t5953 * t13220 + 6.0_f64 * t1668 * t19044 + 3.0_f64 * t1668 * t19047 + 6.0_f64 * t547 * t2061 * t6323 + 6.0_f64 * t547 * t20690 * t2105 + 12.0_f64 * t1279 * t20691;
    t67843
}
