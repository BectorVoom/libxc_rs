//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1185/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1185(t1353: f64, t4135: f64, t1448: f64, t2371: f64, t648: f64, t2319: f64, t670: f64, t10259: f64, t94: f64, t2408: f64, t775: f64, t2394: f64, t890: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t49640 = t4135 * t1353;
    let t49654 = t1448 * t4135;
    let t49693 = t648 * t2371;
    let t49851 = t2319 * t670;
    let t49856 = t94 * t10259;
    let t50066 = t2408 * t775;
    let t51775 = t890 * t2394;
    (t49640, t49654, t49693, t49851, t49856, t50066, t51775)
}
