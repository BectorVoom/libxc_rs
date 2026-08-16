//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1016/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1016(t12567: f64, t3271: f64, t3016: f64, t797: f64, t3262: f64, t3263: f64, t3060: f64, t10997: f64, t3275: f64, t11018: f64, t12434: f64, t12437: f64, t12440: f64, t12443: f64, t12560: f64, t12563: f64, t12565: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12568 = t12567 * t3271;
    let t12569 = t12568 / 4.0_f64;
    let t12570 = t797 * t3016;
    let t12572 = t3262 * t3263 * t12570;
    let t12573 = 3.0_f64 / 4.0_f64 * t12572;
    let t12574 = t797 * t3060;
    let t12576 = t3275 * t10997 * t12574;
    let t12577 = 45.0_f64 / 64.0_f64 * t12576;
    let t12578 = t12434 + 0.15243824895787514157e-3_f64 * t12437 - t11018 - 0.36021158228745895953e-3_f64 * t12440 - 0.72042316457491791906e-3_f64 * t12443 - t12560 + t12563 - t12565 - t12569 - t12573 - t12577;
    (t12569, t12570, t12573, t12574, t12577, t12578)
}
