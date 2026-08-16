//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1197/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1197(t11354: f64, t6113: f64, t918: f64, t4598: f64, t4606: f64, t2880: f64, t6120: f64, t11358: f64, t4614: f64, t2897: f64, t18950: f64, t916: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18979 = t11354 * t6113;
    let t18980 = t18979 * t918;
    let t18982 = t4598 * t4606;
    let t18984 = t2880 * t6120;
    let t18985 = t18984 * t918;
    let t18987 = t11358 * t6113;
    let t18988 = t18987 * t918;
    let t18990 = t4614 * t4606;
    let t18992 = t2897 * t6120;
    let t18993 = t18992 * t918;
    let t18995 = t916 * t18950;
    (t18980, t18982, t18985, t18988, t18990, t18993, t18995)
}
