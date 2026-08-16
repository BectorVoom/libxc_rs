//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1169/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1169(t121204: f64, t1868: f64, t9818: f64, t121232: f64, t1353: f64, t1903: f64, t120956: f64, t1414: f64, t828: f64, t120967: f64, t125627: f64, t247: f64, t3938: f64) -> (f64, f64, f64, f64, f64) {
    let t125662 = t9818 * t121204 * t1868;
    let t125663 = t121232 * t125662;
    let t125668 = t1903 * t1353;
    let t125671 = t120956 * t1414 * t828 * t125668;
    let t125677 = t120967 * t247 * t125627 * t3938;
    (t125662, t125663, t125668, t125671, t125677)
}
