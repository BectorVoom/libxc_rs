//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2879/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2879(t1583: f64, t1940: f64, t39783: f64, t39786: f64, t39791: f64, t39795: f64, t39799: f64, t49958: f64, t49964: f64, t49982: f64, t63160: f64, t76974: f64, t76976: f64) -> f64 {
    let t77386 = -3.0_f64 * t1583 * t1940 * t63160 - t39783 - t39786 - t39791 - t39795 + t39799 - t49958 - t49964 + t49982 + t76974 + t76976;
    t77386
}
