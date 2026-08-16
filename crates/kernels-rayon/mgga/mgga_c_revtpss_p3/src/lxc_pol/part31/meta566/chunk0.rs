//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1977/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1977(t1448: f64, t6816: f64, t1868: f64, t5778: f64, t10309: f64, t607: f64, t2275: f64, t613: f64, t10355: f64, t43: f64, t843: f64, t45963: f64, t6957: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t86771 = t6816 * t1448;
    let t86815 = t1868 * t5778;
    let t92568 = t10309 * t607;
    let t92600 = t613 * t2275;
    let t92605 = t43 * t10355;
    let t92612 = 1232.0_f64 / 27.0_f64 * t843;
    let t92684 = t45963 * t6957;
    (t86771, t86815, t92568, t92600, t92605, t92612, t92684)
}
