//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2624/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2624(t11539: f64, t1174: f64, t22055: f64, t18454: f64, t4889: f64, t1180: f64, t1184: f64, t1714: f64, t18321: f64, t18523: f64, t18550: f64, t18555: f64, t22032: f64, t460: f64, t4928: f64, t4934: f64, t4937: f64, t6138: f64, t73113: f64, t73287: f64, t73290: f64) -> f64 {
    let t73307 = t1174 * t11539 * t22055;
    let t73314 = t4889 * t18454;
    let t73316 = 0.12674897119341563786e-1_f64 * t73113 * t1180 - 0.24444444444444444444e-1_f64 * t18321 * t4937 - 0.9259259259259259259e-4_f64 * t73287 - 0.8333333333333333333e-3_f64 * t73290 - 0.24999999999999999999e-2_f64 * t1174 * t4934 * t18523 * t1714 * t460 - 0.24999999999999999999e-2_f64 * t1174 * t4934 * t6138 * t4928 * t460 + 0.13333333333333333333e-1_f64 * t4889 * t18550 + 0.66666666666666666666e-2_f64 * t4889 * t18555 + 0.7407407407407407407e-3_f64 * t73307 - 0.83333333333333333332e-3_f64 * t1174 * t4934 * t22032 * t1184 * t460 + 0.7407407407407407407e-3_f64 * t73314;
    t73316
}
