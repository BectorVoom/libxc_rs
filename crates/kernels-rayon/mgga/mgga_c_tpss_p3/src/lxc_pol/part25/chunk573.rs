//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 573/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk573(t1114: f64, t3060: f64, t242: f64, t1111: f64, t453: f64, t458: f64, t1141: f64, t2738: f64) -> (f64, f64, f64, f64) {
    let t3061 = t3060 * t1114;
    let t3062 = t242 * t3061;
    let t3063 = t1111 * t3062;
    let t3065 = t453 * t458;
    let t3067 = t1141 * t3065 * t2738;
    (t3062, t3063, t3065, t3067)
}
