//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 924/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk924(t1066: f64, t2927: f64, t1062: f64, t2973: f64, t2972: f64, t398: f64, t393: f64, t1074: f64, t2936: f64, t2976: f64, t3053: f64, t3061: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8776 = t2927 * t1066;
    let t8781 = t1062 * t2973;
    let t8785 = 1.0_f64 / t2972 / t398;
    let t8786 = t393 * t8785;
    let t8787 = t2936 * t1074;
    let t8788 = t8787 * t2976;
    let t8791 = t3053 * t3061;
    (t8776, t8781, t8785, t8786, t8787, t8788, t8791)
}
