//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3537/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3537(t11875: f64, t11922: f64, t19757: f64, t1045: f64, t11144: f64, t11774: f64, t11866: f64, t15586: f64, t15595: f64, t15689: f64, t15701: f64, t15936: f64, t16049: f64, t16095: f64, t1651: f64, t19626: f64, t19864: f64, t20038: f64, t20091: f64, t3115: f64, t3117: f64, t42155: f64, t42328: f64, t42410: f64, t4574: f64, t4900: f64, t54994: f64, t55000: f64, t55122: f64, t55141: f64, t55209: f64, t65060: f64, t66734: f64, t67090: f64) -> f64 {
    let t67152 = t11875 * t11922 * t19757;
    let t67182 = -0.1270341277572436651e-2_f64 * t16095 * t42410 * t1651 * t11144 * t15936 + 0.28582678745379824648e-3_f64 * t67152 - 0.57165357490759649296e-3_f64 * t54994 - 0.3811023832717309953e-3_f64 * t55000 - 0.57165357490759649296e-3_f64 * t55141 * t15586 - 0.47637797908966374413e-3_f64 * t11774 * t55122 * t15595 - 0.57165357490759649296e-3_f64 * t42155 * t19864 + 0.57165357490759649296e-3_f64 * t11774 * t15701 * t67090 - 0.95275595817932748826e-3_f64 * t15689 * t66734 * t4900 * t4574 + 0.57165357490759649296e-3_f64 * t42328 * t55209 * t4900 * t20038 + 0.15244095330869239812e-2_f64 * t16049 * t19626 - 0.85748036236139473944e-3_f64 * t11866 * t20091 - 0.42874018118069736972e-3_f64 * t3115 * t3117 * t65060 * t1045;
    t67182
}
