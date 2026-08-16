//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 809/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk809(t4265: f64, t8995: f64, t140: f64, t299: f64, t9010: f64, t9003: f64, t9007: f64, t695: f64, t8662: f64, t22249: f64, t740: f64, t5439: f64, t9234: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24320 = t4265 * t8995;
    let t24324 = t140 * t299 * t9010;
    let t24374 = t4265 * t9003;
    let t24376 = t4265 * t9007;
    let t24434 = t8662 * t695;
    let t24473 = t22249 * t740;
    let t24561 = t9234 * t5439;
    (t24320, t24324, t24374, t24376, t24434, t24473, t24561)
}
