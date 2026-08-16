//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 711/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk711(t2067: f64, t645: f64, t127: f64, t162: f64, t1948: f64, t2035: f64, t2034: f64, t2022: f64, t2024: f64, t616: f64, t2030: f64, t2037: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6776 = t2067 * t645;
    let t6777 = t6776 * t127;
    let t6778 = t162 * t6777;
    let t6781 = t2035 * t1948;
    let t6782 = t2034 * t6781;
    let t6785 = t2022 * t2024;
    let t6786 = t6785 * t616;
    let t6787 = t2034 * t6786;
    let t6790 = t645 * t2024;
    let t6791 = t6790 * t2067;
    let t6792 = t162 * t6791;
    let t6795 = t2030 * t2037;
    (t6777, t6778, t6781, t6782, t6785, t6786, t6787, t6791, t6792, t6795)
}
