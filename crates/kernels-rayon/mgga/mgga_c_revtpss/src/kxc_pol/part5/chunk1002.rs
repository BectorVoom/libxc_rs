//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1002/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1002(t234: f64, t2735: f64, t10631: f64, t808: f64, t2699: f64, t798: f64, t802: f64, t159: f64, t853: f64, t216: f64, t2729: f64, t794: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10886 = t2735 * t234;
    let t10887 = t808 * t10631;
    let t10888 = t10886 * t10887;
    let t10890 = t2699 * t798;
    let t10891 = t10890 * t802;
    let t10899 = t159 * t853;
    let t10900 = t216 * t10899;
    let t10905 = t794 * t2729;
    (t10886, t10888, t10890, t10891, t10900, t10905)
}
