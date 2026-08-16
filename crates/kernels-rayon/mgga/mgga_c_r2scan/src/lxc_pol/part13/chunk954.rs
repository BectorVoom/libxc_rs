//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 954/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk954(t10903: f64, t3320: f64, t783: f64, t2078: f64, t269: f64, t1060: f64, t10860: f64, t10864: f64, t10867: f64, t10871: f64, t10873: f64, t10876: f64, t10880: f64, t10883: f64, t10887: f64, t10889: f64, t10893: f64, t10896: f64, t10898: f64, t10902: f64) -> (f64, f64, f64) {
    let t10905 = t783 * t10903 * t3320;
    let t10906 = 0.46574606203128791246e-1_f64 * t10905;
    let t10907 = t2078 * t269;
    let t10909 = t783 * t10907 * t1060;
    let t10911 = 0.43341108700271342816e-1_f64 * t10860 + t10864 + t10867 - t10871 - 0.86682217400542685632e-1_f64 * t10873 - 0.43341108700271342816e-1_f64 * t10876 - 0.2600466522016280569e0_f64 * t10880 - 0.13002332610081402845e0_f64 * t10883 + t10887 + 0.5200933044032561138e0_f64 * t10889 + t10893 + t10896 - t10898 - t10902 + t10906 - 0.21831846657716620896e-2_f64 * t10909;
    (t10905, t10907, t10911)
}
