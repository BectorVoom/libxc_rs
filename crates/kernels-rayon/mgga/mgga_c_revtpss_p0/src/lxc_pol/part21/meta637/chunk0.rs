//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2410/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2410(t11010: f64, t689: f64, t779: f64, t2769: f64, t786: f64, t861: f64, t10997: f64, t11007: f64, t252: f64, t11009: f64, t123: f64, t676: f64) -> (f64, f64, f64, f64, f64) {
    let t41063 = t689 * t779 * t11010;
    let t41066 = t786 * t861 * t2769;
    let t41067 = t41066 * t10997;
    let t41070 = t786 * t252 * t11007;
    let t41073 = t41070 * t123 * t676 * t11009;
    (t41063, t41066, t41067, t41070, t41073)
}
