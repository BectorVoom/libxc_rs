//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 829/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk829(t283: f64, t2857: f64, t66: f64, t3298: f64, t994: f64, t4891: f64, t3316: f64, t11132: f64, t126: f64, t373: f64, t828: f64, t1086: f64, t3057: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11852 = 1.0_f64 / t283 / t2857;
    let t11853 = t66 * t11852;
    let t11858 = t994 * t3298;
    let t11859 = t11858 * t4891;
    let t11874 = t994 * t3316;
    let t11875 = t11874 * t4891;
    let t11890 = 0.25925925925925925926e-1_f64 * t11132;
    let t11921 = t126 * t373;
    let t11922 = t828 * t11921;
    let t11926 = t3057 * t1086;
    (t11853, t11858, t11859, t11874, t11875, t11890, t11922, t11926)
}
