//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1081/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1081(t38322: f64, t10655: f64, t10946: f64, t10810: f64, t3429: f64, t3457: f64, t10922: f64, t10992: f64, t158: f64, t2312: f64, t3446: f64, t37428: f64) -> (f64, f64, f64, f64, f64) {
    let t38323 = 0.13010691197123848594e-3_f64 * t38322;
    let t38336 = t10655 * t10946;
    let t38337 = 0.12195059916630011326e-2_f64 * t38336;
    let t38339 = t3429 * t10810 * t3457;
    let t38341 = t10922 * t10946;
    let t38342 = 0.12195059916630011326e-2_f64 * t38341;
    let t38346 = t3446 * t10992 * t158 * t37428 * t2312;
    (t38323, t38337, t38339, t38342, t38346)
}
