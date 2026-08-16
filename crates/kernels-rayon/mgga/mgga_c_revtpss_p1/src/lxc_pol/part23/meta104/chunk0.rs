//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 689/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk689(t243: f64, t2668: f64, t816: f64, t813: f64, t2482: f64, t27: f64, t849: f64) -> (f64, f64) {
    let t2670 = t2668 * t243 * t816;
    let t2672 = 0.13552000749142754193e-3_f64 * t813 * t2670;
    let t2674 = t2482 * t849 * t27;
    (t2672, t2674)
}
