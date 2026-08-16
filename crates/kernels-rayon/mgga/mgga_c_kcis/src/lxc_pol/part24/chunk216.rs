//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 216/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk216(t949: f64, t950: f64, t931: f64, t920: f64, t924: f64) -> (f64, f64, f64, f64) {
    let t951 = t949 * t950;
    let t953 = 1.0_f64 * t931 * t951;
    let t954 = 0.92708333333333333333e-2_f64 * t920;
    let t956 = -t954 - 0.92708333333333333333e-2_f64 * t924;
    (t951, t953, t954, t956)
}
