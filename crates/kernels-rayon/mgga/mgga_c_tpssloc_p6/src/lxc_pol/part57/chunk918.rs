//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 918/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk918(t10143: f64, t8565: f64, t531: f64, t8639: f64, t22716: f64, t8622: f64, t2085: f64, t212: f64, t22642: f64, t6890: f64, t794: f64, t22690: f64, t31618: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t115027 = t8565 * t10143;
    let t115262 = t531 * t8639;
    let t115305 = t22716 * t8622;
    let t115306 = 0.63969658155208805863e-1_f64 * t115305;
    let t115330 = t22642 * t212 * t2085 * t6890;
    let t115331 = 0.82246703342411321824e-2_f64 * t115330;
    let t115352 = t794 * t2085;
    let t115390 = t22642 * t22690 * t31618;
    (t115027, t115262, t115306, t115331, t115352, t115390)
}
