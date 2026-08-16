//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1186/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1186(t120199: f64, t33425: f64, t34904: f64, t33397: f64, t34990: f64, t33404: f64, t1263: f64, t1828: f64, t1042: f64, t1122: f64, t1203: f64, t1214: f64, t124594: f64, t124646: f64, t124780: f64, t1248: f64, t124945: f64, t124984: f64, t1287: f64, t13141: f64, t131416: f64, t2148: f64, t247: f64, t29124: f64, t33401: f64, t33405: f64, t33408: f64, t33461: f64, t33462: f64, t33469: f64, t33478: f64, t34908: f64, t34914: f64, t34939: f64, t34960: f64, t3719: f64, t494: f64, t5245: f64, t5342: f64) -> f64 {
    let t131734 = t33425 * t120199 * t34904;
    let t131745 = t33397 * t34990;
    let t131748 = t33404 * t34990;
    let t131766 = t1263 * t1828;
    let t131771 = 0.3427184259906141157e1_f64 * t124945 * t34914 * t1248 * t1287 - 0.12548651892657985333e-3_f64 * t131734 - 0.3427184259906141157e1_f64 * t124984 * t34939 * t1248 * t1287 - 0.56468933516960933998e-3_f64 * t33405 * t247 * t3719 * t494 * t5245 - 0.30116764542379164798e-2_f64 * t131745 * t33401 + 0.30116764542379164798e-2_f64 * t131748 * t33408 - 0.34271842599061411569e1_f64 * t33469 * t33462 * t34908 * t1214 - 0.51407763898592117355e1_f64 * t33461 * t33478 * t34960 * t1203 - 0.17347256376410398924e1_f64 * t124780 * t29124 + 0.56468933516960933998e-3_f64 * t2148 * t13141 * t131416 * t124646 * t5342 - 0.24791552806034007213e-3_f64 * t124594 * t1042 * t131766 * t1122;
    t131771
}
