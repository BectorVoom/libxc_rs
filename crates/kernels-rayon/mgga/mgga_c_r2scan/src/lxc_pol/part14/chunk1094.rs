//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1094/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1094(t10655: f64, t10946: f64, t10810: f64, t3429: f64, t3457: f64, t10922: f64, t10992: f64, t158: f64, t2312: f64, t3446: f64, t37428: f64, t3428: f64, t3430: f64, t6836: f64) -> (f64, f64, f64, f64, f64) {
    let t38336 = t10655 * t10946;
    let t38339 = t3429 * t10810 * t3457;
    let t38341 = t10922 * t10946;
    let t38346 = t3446 * t10992 * t158 * t37428 * t2312;
    let t38349 = t6836 * t3428 * t3430;
    (t38336, t38339, t38341, t38346, t38349)
}
