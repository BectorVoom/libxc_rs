//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3194/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3194(t15572: f64, t15740: f64, t11697: f64, t18382: f64, t3577: f64, t3575: f64, t62053: f64, t3624: f64, t1229: f64, t1734: f64, t375: f64, t3610: f64) -> (f64, f64, f64, f64, f64) {
    let t66360 = t15740 * t15572;
    let t66363 = t3577 * t11697 * t18382;
    let t66371 = t3575 * t62053;
    let t66372 = t3624 * t66371;
    let t66374 = t375 * t1229 * t1734;
    let t66378 = t3610 * t66371;
    (t66360, t66363, t66372, t66374, t66378)
}
