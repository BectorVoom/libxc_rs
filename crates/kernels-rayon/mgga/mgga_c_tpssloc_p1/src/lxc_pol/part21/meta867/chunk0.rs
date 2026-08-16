//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3164/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3164(t3508: f64, t6218: f64, t1215: f64, t11721: f64, t6224: f64, t15594: f64, t4993: f64, t11692: f64, t11697: f64, t18396: f64, t18400: f64, t3577: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t65464 = t6218 * t3508;
    let t65469 = t6218 * t1215;
    let t65474 = t6224 * t11721;
    let t65479 = t15594 * t4993;
    let t65482 = t11692 * t11697 * t18396;
    let t65485 = t3577 * t11697 * t18400;
    (t65464, t65469, t65474, t65479, t65482, t65485)
}
