//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1166/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1166(t11724: f64, t26278: f64, t11675: f64, t26282: f64, t12550: f64, t2207: f64, t3328: f64, t1058: f64, t9418: f64, t11780: f64, t3606: f64, t10760: f64, t22820: f64, t29279: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43183 = t26278 * t11724;
    let t43185 = t26282 * t11675;
    let t43188 = t2207 * t12550 * t3328;
    let t43191 = t2207 * t1058 * t9418;
    let t43195 = t2207 * t11780 * t3606;
    let t43200 = t22820 * t10760 * t29279;
    (t43183, t43185, t43188, t43191, t43195, t43200)
}
