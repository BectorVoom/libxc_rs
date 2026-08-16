//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 324/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk324(t1061: f64, t1062: f64, t1011: f64, t1017: f64) -> (f64, f64, f64) {
    let t1063 = t1061 * t1062;
    let t1066 = 0.92708333333333333333e-2_f64 * t1011;
    let t1068 = -t1066 + 0.92708333333333333333e-2_f64 * t1017;
    (t1063, t1066, t1068)
}
