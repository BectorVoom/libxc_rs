//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1072/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1072(t1358: f64, t28824: f64, t689: f64, t786: f64, t8086: f64, t1364: f64, t72: f64, t8103: f64, t686: f64, t7284: f64, t26265: f64, t5722: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28825 = t28824 * t1358;
    let t28826 = t689 * t28825;
    let t28837 = t786 * t8086;
    let t28838 = t28837 * t1364;
    let t28844 = t8103 * t72;
    let t28845 = t28844 * t686;
    let t28846 = t7284 * t28845;
    let t28853 = t26265 * t5722;
    (t28825, t28826, t28837, t28838, t28844, t28845, t28846, t28853)
}
