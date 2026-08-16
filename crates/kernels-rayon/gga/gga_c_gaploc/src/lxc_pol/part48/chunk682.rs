//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 682/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk682(t13141: f64, t9438: f64, t2684: f64, t2610: f64, t3431: f64, t2365: f64, t2033: f64, t10007: f64, t1022: f64, t825: f64, t2558: f64, t3464: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13142 = t9438 * t13141;
    let t13143 = t2684 * t13142;
    let t13145 = t2610 * t3431;
    let t13146 = t2365 * t13145;
    let t13147 = t2033 * t13146;
    let t13149 = t10007 * t1022;
    let t13150 = t9438 * t13149;
    let t13151 = t825 * t13150;
    let t13176 = t3464 * t2558;
    (t13142, t13143, t13145, t13146, t13147, t13149, t13150, t13151, t13176)
}
