//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1238/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1238(t25759: f64, t51806: f64, t27799: f64, t50066: f64, t51792: f64, t51775: f64, t1113: f64, t2411: f64, t14365: f64, t11054: f64, t33: f64, t41161: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t94228 = t25759 * t51806;
    let t94231 = t27799 * t50066;
    let t94234 = t27799 * t51792;
    let t94240 = t25759 * t51775;
    let t94245 = t2411 * t1113;
    let t94246 = t94245 * t14365;
    let t94255 = t33 * t11054;
    let t94259 = t25759 * t41161;
    (t94228, t94231, t94234, t94240, t94246, t94255, t94259)
}
