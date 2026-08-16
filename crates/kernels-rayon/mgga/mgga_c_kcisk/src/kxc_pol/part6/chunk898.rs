//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 898/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk898(t652: f64, t743: f64, t719: f64, t717: f64, t415: f64, t2527: f64, t8672: f64, t1801: f64, t11227: f64, t1869: f64, t6697: f64, t5062: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28957 = 1.0_f64 / t652 / t743;
    let t28958 = sigma2 * t28957;
    let t28959 = t28958 * t719;
    let t28960 = t717 * t28959;
    let t28961 = t415 * t28960;
    let t28963 = t8672 * t2527;
    let t28964 = t1801 * t28963;
    let t28965 = t11227 * t28964;
    let t28966 = t1869 * t28965;
    let t28968 = t6697 * t8672;
    let t28969 = t5062 * t28968;
    (t28957, t28958, t28961, t28963, t28966, t28969)
}
