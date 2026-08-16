//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1709/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1709(t18554: f64, t4934: f64, t1178: f64, t16558: f64, t1177: f64, t1184: f64, t460: f64, t6138: f64, t11556: f64, t1174: f64, t1187: f64, t15401: f64, t15405: f64, t15422: f64, t18321: f64, t18536: f64, t18543: f64, t18546: f64, t18550: f64, t3447: f64, t4889: f64, t4913: f64, t4931: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18555 = t4934 * t18554;
    let t18558 = t1178 * t16558;
    let t18559 = t1177 * t18558;
    let t18563 = t6138 * t1184 * t460;
    let t18564 = t4934 * t18563;
    let t18569 = 0.14814814814814814815e-2_f64 * t18536 - 0.81481481481481481481e-2_f64 * t18321 * t1187 + 0.44444444444444444444e-2_f64 * t4889 * t4931 + t11556 + 0.55555555555555555554e-3_f64 * t3447 * t18543 + 0.11111111111111111111e-2_f64 * t3447 * t18546 + t15401 - t15405 + t15422 - 0.16666666666666666666e-2_f64 * t1174 * t18550 - 0.83333333333333333332e-3_f64 * t1174 * t18555 - 0.27777777777777777777e-3_f64 * t1174 * t18559 - 0.83333333333333333332e-3_f64 * t1174 * t18564 + 0.14814814814814814814e-2_f64 * t4889 * t4913;
    (t18555, t18558, t18559, t18563, t18564, t18569)
}
