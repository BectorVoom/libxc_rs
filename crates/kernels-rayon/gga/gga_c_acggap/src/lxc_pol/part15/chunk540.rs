//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 540/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk540(t1004: f64, t1244: f64, t460: f64, t848: f64, t183: f64, t3645: f64, t188: f64, t441: f64, t862: f64, t865: f64, t447: f64, t150: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3842 = 0.19756347548806534796e1_f64 * t1004 * t1244;
    let t3843 = t848 * t460;
    let t3846 = 0.65854491829355115987e0_f64 * t3645 * t183;
    let t3862 = 0.65854491829355115987e0_f64 * t3645 * t188;
    let t3868 = t862 * t441;
    let t3869 = t3868 * t865;
    let t3873 = t447 * t447;
    let t3874 = 1.0_f64 / t3873;
    let t3875 = t150 * t3874;
    (t3842, t3843, t3846, t3862, t3869, t3873, t3874, t3875)
}
