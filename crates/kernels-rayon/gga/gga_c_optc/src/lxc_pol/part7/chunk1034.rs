//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1034/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1034(t1824: f64, t1827: f64, t1788: f64, t1791: f64, t13: f64, t22494: f64, t1792: f64, t6452: f64, t1755: f64, t6454: f64, t1863: f64, t1866: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t22563 = t1824 * t1824;
    let t22566 = t1827 * t1827;
    let t22571 = t1788 * t1788;
    let t22574 = t1791 * t1791;
    let t22578 = 0.24954977986735470917e5_f64 * t13 / t22571 * t22494 / t22574;
    let t22581 = 0.57894567559743977359e3_f64 * t6452 * t22494 * t1792;
    let t22593 = 0.620700176468474021e4_f64 * t13 / t1788 / t1755 * t22494 * t6454;
    let t22597 = t1863 * t1863;
    let t22598 = 1.0_f64 / t22597;
    let t22600 = t1866 * t1866;
    (t22563, t22566, t22578, t22581, t22593, t22598, t22600)
}
