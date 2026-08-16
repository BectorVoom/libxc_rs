//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3502/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3502(t15669: f64, t16088: f64, t380: f64, t1042: f64, t1063: f64, t11703: f64, t11994: f64, t15707: f64, t16091: f64, t16095: f64, t16096: f64, t16144: f64, t18908: f64, t19672: f64, t19693: f64, t3106: f64, t4801: f64, t4837: f64, t51958: f64, t53690: f64, t65947: f64, t66017: f64, t66022: f64, t66024: f64, t66029: f64, t66037: f64, t66043: f64) -> f64 {
    let t66047 = t15669 * t380 * t16088;
    let t66054 = 0.28582678745379824648e-3_f64 * t66017 - 0.47637797908966374414e-3_f64 * t11994 * t19693 - 0.47637797908966374413e-4_f64 * t66022 + 0.15244095330869239812e-2_f64 * t66024 - 0.67751534803863288055e-2_f64 * t3106 * t19672 - 0.95275595817932748827e-4_f64 * t66029 - 0.34299214494455789578e-2_f64 * t1063 * t1042 * t51958 * t65947 + 0.57165357490759649296e-3_f64 * t15707 * t16144 - 0.57165357490759649296e-3_f64 * t4837 * t1042 * t4801 * t66037 + 0.3811023832717309953e-3_f64 * t66043 + 0.3811023832717309953e-3_f64 * t53690 + 0.11433071498151929859e-2_f64 * t66047 * t16091 + 0.28582678745379824648e-2_f64 * t16095 * t11703 * t18908 * t16096;
    t66054
}
