//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2724/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2724(t3666: f64, t6594: f64, t17283: f64, t5362: f64, t1222: f64, t140: f64, t21209: f64, t21213: f64, t3685: f64, t12865: f64, t5436: f64, t3671: f64, t371: f64, t6609: f64, t676: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t70469 = t3666 * t6594;
    let t70476 = t17283 * t5362;
    let t70491 = t1222 * t140 * t21209;
    let t70493 = t21213 * t3685;
    let t70496 = t5436 * t12865;
    let t70511 = t3671 * t371 * t676 * t6609;
    (t70469, t70476, t70491, t70493, t70496, t70511)
}
