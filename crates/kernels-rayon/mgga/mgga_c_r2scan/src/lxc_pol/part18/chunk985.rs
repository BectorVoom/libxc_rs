//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 985/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk985(t10710: f64, t8128: f64, t10768: f64, t10781: f64, t2547: f64, t2207: f64, t3336: f64, t3606: f64, t1060: f64, t2526: f64, t1058: f64, t3333: f64, t7601: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11816 = t10710 * t8128;
    let t11817 = t10768 * t11816;
    let t11819 = t10781 * t2547;
    let t11822 = t2207 * t3336 * t3606;
    let t11824 = t1060 * t2526;
    let t11826 = t2207 * t1058 * t11824;
    let t11831 = t7601 * t3333;
    (t11816, t11817, t11819, t11822, t11824, t11826, t11831)
}
