//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 717/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk717(t596: f64, t6838: f64, t6480: f64, t6484: f64, t6488: f64, t6492: f64, t6816: f64, t6819: f64, t6823: f64, t6827: f64, t6829: f64, t6832: f64, t6834: f64, t6836: f64) -> (f64, f64) {
    let t6840 = 0.56969282336565386482e-3_f64 * t596 * t6838;
    let t6841 = t6816 - t6819 - t6480 - t6484 + t6488 - t6823 + t6827 + t6829 + t6832 + t6834 + t6836 + t6492 - t6840;
    (t6840, t6841)
}
