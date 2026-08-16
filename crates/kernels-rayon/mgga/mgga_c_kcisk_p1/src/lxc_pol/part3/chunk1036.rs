//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 1036/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk1036(t15316: f64, t73: f64, t2950: f64, t879: f64, t2981: f64, t15292: f64, t15294: f64, t15296: f64, t15298: f64, t15302: f64, t15304: f64, t15306: f64, t15308: f64) -> (f64, f64, f64, f64) {
    let t15317 = t73 * t15316;
    let t15318 = t2950 * t879;
    let t15319 = t15318 * t2981;
    let t15330 = -0.50638e1_f64 * t15292 + 0.16879333333333333333e1_f64 * t15294 - 0.19692555555555555555e1_f64 * t15296 - 0.93011851851851851854e0_f64 * t15298 + 0.27303333333333333333e0_f64 * t15302 - 0.27303333333333333333e0_f64 * t15304 - 0.3185388888888888889e0_f64 * t15306 - 0.36514074074074074075e0_f64 * t15308;
    (t15317, t15318, t15319, t15330)
}
