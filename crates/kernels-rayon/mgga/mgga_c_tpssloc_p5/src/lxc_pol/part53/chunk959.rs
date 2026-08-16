//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 959/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk959(t32244: f64, t9231: f64, t116904: f64, t2240: f64, t12461: f64, t8807: f64, t111: f64, t32262: f64, t3701: f64, t8803: f64, t115305: f64, t115330: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t116947 = t9231 * t32244;
    let t116954 = t2240 * t116904;
    let t117006 = t8807 * t12461;
    let t117014 = t32262 * t111;
    let t117084 = t8803 * t3701;
    let t117128 = 0.25587863262083522346e0_f64 * t115305;
    let t117133 = 0.3289868133696452873e-1_f64 * t115330;
    (t116947, t116954, t117006, t117014, t117084, t117128, t117133)
}
