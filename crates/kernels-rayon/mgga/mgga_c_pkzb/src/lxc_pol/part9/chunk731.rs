//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 731/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk731(t5089: f64, t555: f64, t12: f64, t137: f64, t1643: f64, t439: f64) -> (f64, f64, f64) {
    let t5091 = 0.10389515463408878255e3_f64 * t555 * t5089;
    let t5093 = 1.0_f64 / t137 / t12;
    let t5094 = t1643 * t439;
    (t5091, t5093, t5094)
}
