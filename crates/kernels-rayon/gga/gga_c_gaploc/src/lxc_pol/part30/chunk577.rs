//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 577/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk577(t3197: f64, t286: f64, t3092: f64, t708: f64, t1687: f64, t3098: f64, t129: f64, t1692: f64, t1685: f64, t3097: f64, t3091: f64, t713: f64) -> (f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t3198 = 0.38342925953920749676e0_f64 * t3197;
    let t3216 = t3092 * t286 * t708;
    let t3218 = t3098 * t1687;
    let t3220 = t1692 * t129;
    let t3221 = t3097 * t1685;
    let t3222 = t3221 * pi;
    let t3223 = t3220 * t3222;
    let t3225 = t713 * t3091;
    (t3198, t3216, t3218, t3220, t3223, t3225)
}
