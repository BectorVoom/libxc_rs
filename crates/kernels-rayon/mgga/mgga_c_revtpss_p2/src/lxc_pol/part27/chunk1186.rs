//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1186/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1186(t2832: f64, t890: f64, t2430: f64, t1298: f64, t3794: f64, t10259: f64, t93: f64, t10301: f64, t607: f64, t10309: f64, t1927: f64, t2248: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t51792 = t890 * t2832;
    let t51806 = t2430 * t890;
    let t60126 = t3794 * t1298;
    let t60551 = t93 * t10259;
    let t92565 = t10301 * t607;
    let t92568 = t10309 * t607;
    let t92569 = t1927 * t2248;
    (t51792, t51806, t60126, t60551, t92565, t92568, t92569)
}
