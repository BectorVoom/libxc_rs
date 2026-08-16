//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1134/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1134(t14365: f64, t94245: f64, t11054: f64, t33: f64, t25759: f64, t41161: f64, t1113: f64, t2394: f64, t3351: f64, t890: f64, t10818: f64, t27763: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t94246 = t94245 * t14365;
    let t94255 = t33 * t11054;
    let t94259 = t25759 * t41161;
    let t94262 = t1113 * t2394;
    let t94276 = t3351 * t890;
    let t94280 = t27763 * t10818;
    (t94246, t94255, t94259, t94262, t94276, t94280)
}
