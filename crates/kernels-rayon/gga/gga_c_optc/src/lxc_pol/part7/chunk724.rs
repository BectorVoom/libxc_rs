//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 724/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk724(t2093: f64, t654: f64, t141: f64, t6560: f64, t659: f64, t2080: f64, t661: f64, t1948: f64, t630: f64, t629: f64, t2089: f64, t104: f64, t137: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6901 = t654 * t2093;
    let t6904 = t659 * t141 * t6560;
    let t6907 = t2080 * t661;
    let t6909 = t630 * t1948;
    let t6910 = t629 * t6909;
    let t6913 = t654 * t2089;
    let t6915 = t137 * t104;
    (t6901, t6904, t6907, t6909, t6910, t6913, t6915)
}
