//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 421/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk421(t15: f64, t3193: f64, t60: f64, t989: f64, t816: f64, t183: f64, t20: f64, t21: f64, t963: f64, t151: f64, t1014: f64, t142: f64, t955: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3194 = t3193 * t15;
    let t3199 = t60 * t989;
    let t3200 = t3199 * t816;
    let t3201 = t183 * t20;
    let t3203 = t3201 * t21 * t963;
    let t3206 = t15 * t151;
    let t3207 = t1014 * t3206;
    let t3208 = t142 * t955;
    (t3194, t3199, t3200, t3201, t3203, t3206, t3207, t3208)
}
