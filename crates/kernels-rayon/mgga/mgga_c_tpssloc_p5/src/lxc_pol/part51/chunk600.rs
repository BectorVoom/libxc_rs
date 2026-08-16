//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 600/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk600(t4613: f64, t4656: f64, t349: f64, t1626: f64, t225: f64, t1065: f64, t1634: f64, t3174: f64, t1057: f64, t4639: f64, t1022: f64, t3188: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4657 = t4613 + t4656;
    let t4658 = t349 * t4657;
    let t4660 = t1626 * t225;
    let t4664 = t1634 * t1065;
    let t4665 = t3174 * t4664;
    let t4669 = t4639 * t1057;
    let t4673 = t3188 * t1022;
    (t4657, t4658, t4660, t4664, t4665, t4669, t4673)
}
