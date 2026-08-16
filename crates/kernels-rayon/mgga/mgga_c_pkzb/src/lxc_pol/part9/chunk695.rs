//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 695/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk695(t2363: f64, t3258: f64, t2970: f64, t3187: f64, t133: f64, t3199: f64, t945: f64, t2393: f64, t3207: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3259 = t2363 * t3258;
    let t3260 = t2970 * t3187;
    let t3265 = t3199 * t133;
    let t3266 = t3265 * t945;
    let t3269 = t2393 * t3258;
    let t3270 = t2970 * t3207;
    (t3259, t3260, t3265, t3266, t3269, t3270)
}
