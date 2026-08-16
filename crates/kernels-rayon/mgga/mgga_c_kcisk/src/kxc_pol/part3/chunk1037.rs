//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1037/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1037(t15330: f64, t880: f64, t2977: f64, t861: f64, t73: f64, t2980: f64, t88: f64, t15318: f64, t3011: f64, t98: f64, t15283: f64, t3015: f64) -> (f64, f64, f64, f64) {
    let t15331 = t15330 * t880;
    let t15335 = 1.0_f64 / t2977 / t861;
    let t15336 = t73 * t15335;
    let t15338 = 1.0_f64 / t2980 / t88;
    let t15339 = t15318 * t15338;
    let t15343 = 1.0_f64 / t3011 / t98;
    let t15345 = t15343 * t15283 * t3015;
    (t15331, t15336, t15339, t15345)
}
