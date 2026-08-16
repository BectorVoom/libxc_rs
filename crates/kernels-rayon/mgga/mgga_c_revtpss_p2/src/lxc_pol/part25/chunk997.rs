//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 997/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk997(t283: f64, t2857: f64, t66: f64, t11145: f64, t247: f64, t3298: f64, t994: f64, t4891: f64, t3154: f64, t999: f64, t11659: f64, t3117: f64) -> (f64, f64, f64, f64) {
    let t11852 = 1.0_f64 / t283 / t2857;
    let t11853 = t66 * t11852;
    let t11855 = t247 * t11853 * t11145;
    let t11858 = t994 * t3298;
    let t11859 = t11858 * t4891;
    let t11860 = t3154 * t999;
    let t11861 = t11659 * t11860;
    let t11862 = t3117 * t11861;
    (t11855, t11858, t11859, t11862)
}
