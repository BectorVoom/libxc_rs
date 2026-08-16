//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2125/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2125(t1568: f64, t2886: f64, t2860: f64, t4408: f64, t10770: f64, t1561: f64, t2791: f64, t4351: f64, t10660: f64, t1543: f64, t10756: f64, t300: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t49422 = t2886 * t1568;
    let t49427 = t4408 * t2860;
    let t49430 = t1561 * t10770;
    let t49486 = t4351 * t2791;
    let t49489 = t1543 * t10660;
    let t49513 = t300 * t10756;
    (t49422, t49427, t49430, t49486, t49489, t49513)
}
