//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2294/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2294(t18375: f64, t3536: f64, t11697: f64, t18968: f64, t3577: f64, t11539: f64, t1174: f64, t18232: f64, t18215: f64, t11665: f64, t18371: f64, t15569: f64, t15572: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t66554 = t3536 * t18375;
    let t66566 = t3577 * t11697 * t18968;
    let t66571 = t1174 * t11539 * t18232;
    let t66575 = t1174 * t11539 * t18215;
    let t66597 = t11665 * t18371;
    let t66599 = t15569 * t15572;
    (t66554, t66566, t66571, t66575, t66597, t66599)
}
