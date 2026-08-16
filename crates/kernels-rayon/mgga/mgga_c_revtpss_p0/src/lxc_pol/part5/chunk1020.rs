//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1020/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1020(t1025: f64, t11817: f64, t271: f64, t2857: f64, t283: f64, t3298: f64, t994: f64, t4891: f64, t3154: f64, t999: f64, t1086: f64, t3046: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11818 = t1025 * t11817;
    let t11821 = 1.0_f64 / t271 / t2857;
    let t11852 = 1.0_f64 / t283 / t2857;
    let t11858 = t994 * t3298;
    let t11859 = t11858 * t4891;
    let t11860 = t3154 * t999;
    let t11865 = t3046 * t1086;
    (t11818, t11821, t11852, t11859, t11860, t11865)
}
