//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 415/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk415(t1281: f64, t204: f64, t208: f64, t648: f64, t655: f64) -> (f64, f64, f64) {
    let t1830 = t204 * t1281 * t208;
    let t1831 = 0.23744444444444444444e-1_f64 * t1830;
    let t1833 = t204 * t648 * t655;
    (t1830, t1831, t1833)
}
