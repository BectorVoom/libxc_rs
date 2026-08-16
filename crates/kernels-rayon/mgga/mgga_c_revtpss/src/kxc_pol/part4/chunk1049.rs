//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1049/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1049(t11858: f64, t4891: f64, t1086: f64, t3046: f64, t3090: f64, t3316: f64, t994: f64, t1016: f64, t697: f64, t1011: f64, t1010: f64, t2270: f64) -> (f64, f64, f64, f64, f64) {
    let t11859 = t11858 * t4891;
    let t11865 = t3046 * t1086;
    let t11866 = t11865 * t3090;
    let t11874 = t994 * t3316;
    let t11875 = t11874 * t4891;
    let t11880 = t697 * t1016;
    let t11881 = t1011 * t11880;
    let t11883 = t2270 * t1010;
    (t11859, t11866, t11875, t11881, t11883)
}
