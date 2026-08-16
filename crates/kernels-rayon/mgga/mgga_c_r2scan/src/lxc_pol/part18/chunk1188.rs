//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1188/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1188(t11780: f64, t2207: f64, t3606: f64, t10760: f64, t22820: f64, t29279: f64, t29726: f64, t6535: f64, t11720: f64, t26282: f64, t1058: f64, t1060: f64, t8629: f64) -> (f64, f64, f64, f64, f64) {
    let t43195 = t2207 * t11780 * t3606;
    let t43200 = t22820 * t10760 * t29279;
    let t43203 = t6535 * t10760 * t29726;
    let t43205 = t26282 * t11720;
    let t43209 = t2207 * t1058 * t1060 * t8629;
    (t43195, t43200, t43203, t43205, t43209)
}
