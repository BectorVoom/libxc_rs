//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1129/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1129(t16534: f64, t7122: f64, t16382: f64, t7110: f64, t16474: f64, t23077: f64, t16483: f64, t7037: f64, t16402: f64, t16543: f64, t9917: f64, t16540: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t48866 = t7122 * t16534;
    let t48875 = t7110 * t16382;
    let t48904 = t23077 * t16474;
    let t48906 = t7037 * t16483;
    let t48922 = t7110 * t16402;
    let t48924 = t9917 * t16543;
    let t48960 = t7122 * t16540;
    (t48866, t48875, t48904, t48906, t48922, t48924, t48960)
}
