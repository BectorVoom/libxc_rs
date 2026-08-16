//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1173/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1173(t32195: f64, t32206: f64, t3936: f64, t5591: f64, t121204: f64, t1868: f64, t9818: f64, t121232: f64, t1353: f64, t1903: f64, t120956: f64, t1414: f64, t828: f64) -> (f64, f64, f64, f64, f64) {
    let t125659 = t32206 * t3936 * t32195 * t5591;
    let t125662 = t9818 * t121204 * t1868;
    let t125663 = t121232 * t125662;
    let t125668 = t1903 * t1353;
    let t125671 = t120956 * t1414 * t828 * t125668;
    (t125659, t125662, t125663, t125668, t125671)
}
