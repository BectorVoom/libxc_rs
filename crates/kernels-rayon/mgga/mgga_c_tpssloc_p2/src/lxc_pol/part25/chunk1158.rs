//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1158/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1158(t6637: f64, t6638: f64, t81984: f64, t9458: f64, t23159: f64, t23168: f64, t1888: f64, t232: f64, t40909: f64, t6646: f64, t23177: f64, t6579: f64) -> (f64, f64, f64, f64) {
    let t81987 = t81984 * t6637 * t6638 * t9458;
    let t81989 = t23168 * t23159;
    let t82003 = t1888 * t6646 * t40909 * t232;
    let t82005 = t6579 * t23177;
    (t81987, t81989, t82003, t82005)
}
