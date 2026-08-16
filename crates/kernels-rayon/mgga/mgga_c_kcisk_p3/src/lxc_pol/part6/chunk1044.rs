//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1044/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1044(t1341: f64, t31133: f64, t1415: f64, t1411: f64, t5606: f64, t8180: f64, t1339: f64, t2231: f64, t7906: f64, t3785: f64, t2152: f64, t1450: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31134 = t1341 * t31133;
    let t31135 = t1415 * t31134;
    let t31136 = t1411 * t31135;
    let t31138 = t5606 * t8180;
    let t31139 = t1339 * t31138;
    let t31141 = t7906 * t2231;
    let t31142 = t1341 * t31141;
    let t31143 = t3785 * t31142;
    let t31144 = t1411 * t31143;
    let t31146 = t7906 * t2152;
    let t31147 = t1450 * t31146;
    (t31136, t31139, t31141, t31144, t31146, t31147)
}
