//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1674/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1674(t12555: f64, t1756: f64, t3497: f64, t16710: f64, t16712: f64, t12297: f64, t12299: f64, t12301: f64, t12303: f64, t12397: f64, t16706: f64, t16708: f64, t16717: f64, t16722: f64, t16727: f64, t16731: f64, t16735: f64, t16740: f64, t16744: f64, t16748: f64) -> (f64, f64) {
    let t16997 = t1756 * t12555;
    let t16998 = t16997 * t3497;
    let t17010 = 0.2283111111111111111e-1_f64 * t16710;
    let t17011 = 0.11415555555555555555e-1_f64 * t16712;
    let t17020 = -t12397 + 0.1522074074074074074e-1_f64 * t12297 + 0.38051851851851851851e-2_f64 * t12299 - 0.11415555555555555555e-1_f64 * t12301 - 0.57077777777777777777e-2_f64 * t12303 + 0.76103703703703703702e-2_f64 * t16706 + 0.76103703703703703701e-2_f64 * t16708 - t17010 - t17011 + 0.19025925925925925925e-1_f64 * t16717 - 0.68493333333333333331e-1_f64 * t16722 - 0.2283111111111111111e-1_f64 * t16727 - 0.11415555555555555555e-1_f64 * t16731 + 0.10274e0_f64 * t16735 + 0.68493333333333333332e-1_f64 * t16740 + 0.34246666666666666666e-1_f64 * t16744 + 0.17123333333333333333e-1_f64 * t16748;
    (t16998, t17020)
}
