//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1304/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1304(t3801: f64, t6748: f64, t1209: f64, t6695: f64, t460: f64, t1214: f64, t6587: f64, t1211: f64, t6744: f64, t1277: f64, t1294: f64, t6573: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20692 = t6748 * t3801;
    let t20697 = t1209 * t6695;
    let t20700 = t460 * t6695;
    let t20703 = t6587 * t1214;
    let t20704 = t1211 * t20703;
    let t20709 = t6744 * t1214;
    let t20710 = t1277 * t20709;
    let t20714 = t1277 * t6573 * t1294;
    (t20692, t20697, t20700, t20703, t20704, t20710, t20714)
}
