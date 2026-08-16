//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 126/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk126(t336: f64, t339: f64, t342: f64, t346: f64) -> (f64, f64, f64) {
    let t374 = 0.51785e1_f64 * t339 + 0.905775e0_f64 * t336 + 0.1100325e0_f64 * t342 + 0.1241775e0_f64 * t346;
    let t377 = 1.0_f64 + 0.29608749977793437516e2_f64 / t374;
    let t378 = f64::ln(t377);
    (t374, t377, t378)
}
