//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1391/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1391(t3247: f64, t39103: f64, t1113: f64, t136: f64, t11545: f64, t241: f64, t3241: f64, t39097: f64, t11229: f64, t699: f64, t11232: f64, t11219: f64, t43732: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t43757 = t3247 * t39103;
    let t43759 = t136 * t1113 * t43757;
    let t43761 = t241 * t11545;
    let t43762 = t3241 * t3241;
    let t43763 = 1.0_f64 / t43762;
    let t43764 = t43763 * t39097;
    let t43766 = t136 * t43761 * t43764;
    let t43768 = t699 * t11229;
    let t43770 = t699 * t11232;
    let t43773 = t136 * t11219 * t43732;
    (t43757, t43759, t43763, t43764, t43766, t43768, t43770, t43773)
}
