//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 271/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk271(t837: f64, t879: f64, t234: f64, t860: f64, t213: f64, t820: f64, t873: f64, t878: f64) -> f64 {
    let t880 = t879 * t837;
    let t883 = t234 * t860;
    let t886 = -t873 + t878 - 0.65854491829355115987e0_f64 * t820 * t880 + 0.65854491829355115987e0_f64 * t213 * t883;
    t886
}
