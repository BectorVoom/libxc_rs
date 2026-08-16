//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2039/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2039(t23034: f64, t6546: f64, t23037: f64, t131: f64, t845: f64, t23159: f64, t23168: f64, t23177: f64, t6579: f64, t23143: f64, t6649: f64, t22999: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t81979 = t6546 * t23034;
    let t81980 = t81979 * t23037;
    let t81982 = t845 * t131;
    let t81989 = t23168 * t23159;
    let t82005 = t6579 * t23177;
    let t82011 = t23143 * t6649;
    let t82013 = t6579 * t22999;
    (t81979, t81980, t81982, t81989, t82005, t82011, t82013)
}
