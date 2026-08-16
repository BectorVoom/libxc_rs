//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 736/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk736(t11402: f64, t11403: f64, t1824: f64, t1825: f64, t4684: f64, t7055: f64, t1648: f64, t1814: f64, t4658: f64, t4629: f64, t10459: f64, t707: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11405 = t11402 * t11403 * t1824;
    let t11408 = t1825 * t4684;
    let t11409 = t7055 * t11408;
    let t11412 = t1814 * t1648;
    let t11413 = t11412 * t4658;
    let t11414 = t4629 * t11413;
    let t11417 = t10459 * t707;
    (t11405, t11408, t11409, t11413, t11414, t11417)
}
