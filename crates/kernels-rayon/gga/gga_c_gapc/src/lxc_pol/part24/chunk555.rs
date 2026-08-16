//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 555/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk555(t3254: f64, t3255: f64, t1061: f64, t2452: f64, t2456: f64, t3239: f64, t1055: f64, t644: f64, t311: f64, t442: f64, t906: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3256 = t3254 * t3255;
    let t3258 = t1061 * t2452;
    let t3259 = t3239 * t2456;
    let t3260 = t3258 * t3259;
    let t3271 = t1055 * t644;
    let t3272 = t311 * t3271;
    let t3273 = t442 * t906;
    (t3256, t3258, t3259, t3260, t3271, t3272, t3273)
}
