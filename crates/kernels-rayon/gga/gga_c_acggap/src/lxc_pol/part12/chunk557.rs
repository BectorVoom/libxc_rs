//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 557/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk557(t460: f64, t848: f64, t183: f64, t3645: f64, t1265: f64, t857: f64, t1210: f64, t315: f64, t323: f64, t188: f64, t119: f64, t441: f64, t862: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3843 = t848 * t460;
    let t3846 = 0.65854491829355115987e0_f64 * t3645 * t183;
    let t3856 = t857 * t1265;
    let t3858 = t315 * t1210;
    let t3859 = t3858 * t323;
    let t3862 = 0.65854491829355115987e0_f64 * t3645 * t188;
    let t3865 = t119 * t1210;
    let t3868 = t862 * t441;
    (t3843, t3846, t3856, t3859, t3862, t3865, t3868)
}
