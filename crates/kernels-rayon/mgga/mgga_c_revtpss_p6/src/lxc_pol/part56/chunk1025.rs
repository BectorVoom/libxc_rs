//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1025/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1025(t1811: f64, t7642: f64, t1214: f64, t1769: f64, t1518: f64, t1936: f64, t670: f64, t8151: f64, t84: f64, t8440: f64, t11064: f64, t8489: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t105364 = t7642 * t1811;
    let t105460 = t1769 * t1214;
    let t105823 = t1518 * t1936;
    let t111734 = t8151 * t670;
    let t119457 = t8440 * t84;
    let t119675 = t8489 * t11064;
    (t105364, t105460, t105823, t111734, t119457, t119675)
}
