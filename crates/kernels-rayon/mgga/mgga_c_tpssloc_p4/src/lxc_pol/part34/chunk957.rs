//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 957/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk957(t18427: f64, t4919: f64, t11547: f64, t20234: f64, t11546: f64, t1174: f64, t15265: f64, t1710: f64, t1717: f64, t18321: f64, t22035: f64, t22041: f64, t22047: f64, t22052: f64, t22056: f64, t22060: f64, t22063: f64, t22066: f64, t22069: f64, t22072: f64, t3447: f64, t4889: f64, t6120: f64, t6141: f64, t6147: f64) -> f64 {
    let t22075 = t4919 * t18427;
    let t22081 = t11547 * t20234;
    let t22082 = t11546 * t22081;
    let t22085 = -0.24444444444444444444e-1_f64 * t18321 * t1717 + 0.66666666666666666666e-2_f64 * t4889 * t6141 + 0.66666666666666666666e-2_f64 * t4889 * t6147 - 0.83333333333333333332e-3_f64 * t1174 * t22035 - 0.83333333333333333332e-3_f64 * t1174 * t22041 - 0.81481481481481481478e-2_f64 * t18321 * t1710 - 0.27777777777777777777e-3_f64 * t1174 * t22047 - 0.24999999999999999999e-2_f64 * t1174 * t22052 + 0.22222222222222222221e-2_f64 * t1174 * t22056 - 0.16666666666666666666e-2_f64 * t1174 * t22060 + 0.11111111111111111111e-2_f64 * t3447 * t22063 - 0.11111111111111111111e-2_f64 * t3447 * t22066 + 0.83333333333333333331e-3_f64 * t3447 * t22069 + 0.83333333333333333331e-3_f64 * t3447 * t22072 + 0.16666666666666666666e-2_f64 * t3447 * t22075 + 0.14814814814814814814e-2_f64 * t15265 - 0.29629629629629629629e-2_f64 * t4889 * t6120 - 0.86419753086419753084e-3_f64 * t1174 * t22082;
    t22085
}
