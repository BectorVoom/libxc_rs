//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 931/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk931(t151: f64, t7731: f64, t950: f64, t3378: f64, t7560: f64, t30049: f64, t7461: f64, t2104: f64, t7610: f64, t1113: f64, t7736: f64, t377: f64, t7732: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31811 = t151 * t7731 * t950;
    let t31824 = t3378 * t7560;
    let t31839 = t30049 * t7461;
    let t31849 = t7610 * t2104;
    let t31855 = t7736 * t1113;
    let t31863 = t377 * t7732;
    (t31811, t31824, t31839, t31849, t31855, t31863)
}
