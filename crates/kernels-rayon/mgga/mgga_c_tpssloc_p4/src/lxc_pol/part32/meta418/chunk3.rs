//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1619/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1619(t11668: f64, t19015: f64, t18232: f64, t3440: f64, t1017: f64, t6163: f64, t1210: f64, t1207: f64, t11665: f64, t11678: f64, t1174: f64, t11834: f64, t1218: f64, t15569: f64, t15717: f64, t15719: f64, t15722: f64, t15740: f64, t18997: f64, t19002: f64, t19005: f64, t19010: f64, t3577: f64, t4889: f64, t4950: f64, t4954: f64, t4969: f64, t5046: f64, t6192: f64) -> (f64, f64, f64) {
    let t19016 = t11668 * t19015;
    let t19019 = t3440 * t18232;
    let t19024 = t6163 * t1017;
    let t19025 = t1210 * t19024;
    let t19026 = t1207 * t19025;
    let t19029 = t15569 * t4950 / 432.0_f64 - t11665 * t6192 / 2304.0_f64 + t4889 * t5046 / 54.0_f64 - t1174 * t18997 / 288.0_f64 - t11678 * t19002 / 1152.0_f64 + t11834 - t1174 * t19005 / 48.0_f64 + t4889 * t4969 / 27.0_f64 - t1174 * t19010 / 144.0_f64 - t15740 * t4954 / 2304.0_f64 + 5.0_f64 / 6912.0_f64 * t3577 * t19016 + t1174 * t19019 / 216.0_f64 + t15717 / 1296.0_f64 - t15719 / 6912.0_f64 - t15722 + 19.0_f64 / 1728.0_f64 * t19026 * t1218;
    (t19016, t19024, t19029)
}
