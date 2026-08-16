//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1176/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1176(t1036: f64, t17744: f64, t17422: f64, t3020: f64, t17500: f64, t3061: f64, t1085: f64, t17360: f64, t1066: f64, t17777: f64, t18190: f64, t34029: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t52890 = t17744 * t1036;
    let t53039 = t17422 * t3020;
    let t53108 = t17500 * t3061;
    let t53152 = t17360 * t1085;
    let t53155 = t17777 * t1066;
    let t53260 = t34029 * t18190;
    (t52890, t53039, t53108, t53152, t53155, t53260)
}
