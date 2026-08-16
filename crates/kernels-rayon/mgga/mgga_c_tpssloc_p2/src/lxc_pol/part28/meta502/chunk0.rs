//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1736/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1736(t5: f64, t26938: f64, t26964: f64, t112: f64, t24990: f64, t7170: f64, t24432: f64, t25988: f64, t2035: f64, t671: f64) -> (f64, f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t26966 = piecewise3(t8, 0.0_f64, t26938 + t26964);
    let t26967 = t26966 * t112;
    let t26969 = t7170 * t24990;
    let t26974 = t24432 * t25988;
    let t26977 = t2035 * t671;
    (t26966, t26967, t26969, t26974, t26977)
}
