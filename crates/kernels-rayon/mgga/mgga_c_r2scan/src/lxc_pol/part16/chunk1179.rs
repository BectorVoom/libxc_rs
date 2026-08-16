//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1179/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1179(t10710: f64, t30428: f64, t37712: f64, t10768: f64, t29194: f64, t29177: f64, t37658: f64, t11816: f64, t39409: f64, t3308: f64, t37652: f64, t8784: f64) -> (f64, f64, f64, f64, f64) {
    let t43097 = t37712 * t10710 * t30428;
    let t43100 = t10768 * t10710 * t29194;
    let t43103 = t37658 * t10710 * t29177;
    let t43105 = t39409 * t11816;
    let t43108 = t37652 * t3308 * t8784;
    (t43097, t43100, t43103, t43105, t43108)
}
