//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 422/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk422(t1667: f64, t550: f64, t46: f64, t512: f64, t552: f64, t637: f64) -> (f64, f64, f64, f64, f64) {
    let t1669 = 0.24415263074675393405e-3_f64 * t550 * t1667;
    let t1670 = t512 * t46;
    let t1671 = t1670 * t552;
    let t1672 = 0.36622894612013090108e-3_f64 * t1671;
    let t1673 = t637 * t637;
    (t1669, t1670, t1671, t1672, t1673)
}
