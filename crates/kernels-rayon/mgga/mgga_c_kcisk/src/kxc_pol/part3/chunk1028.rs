//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1028/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1028(t15175: f64, t979: f64, t1002: f64, t12705: f64, t3077: f64, t3133: f64, t3176: f64, t116: f64, t12699: f64, t12698: f64, t12809: f64, t142: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15176 = t979 * t15175;
    let t15178 = t12705 * t1002;
    let t15179 = t979 * t15178;
    let t15181 = t3077 * t3133;
    let t15183 = t3077 * t3176;
    let t15185 = t116 * t12699;
    let t15186 = t12698 * t15185;
    let t15187 = t979 * t15186;
    let t15189 = t142 * t12809;
    (t15176, t15179, t15181, t15183, t15187, t15189)
}
