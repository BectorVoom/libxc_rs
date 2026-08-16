//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1169/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1169(t11947: f64, t8900: f64, t43706: f64, t8904: f64, t2174: f64, t7415: f64, t2169: f64, t7426: f64, t1395: f64, t8927: f64, t32649: f64, t576: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t118233 = t8900 * t11947;
    let t118251 = t8904 * t43706;
    let t118335 = t7415 * t2174;
    let t118337 = t2169 * t7426;
    let t118345 = t1395 * t8927;
    let t118347 = t576 * t32649;
    (t118233, t118251, t118335, t118337, t118345, t118347)
}
