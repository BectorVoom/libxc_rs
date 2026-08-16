//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1432/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1432(t1164: f64, t43689: f64, t43692: f64, t78287: f64, t18622: f64, t64451: f64, t21833: f64, t4869: f64, t5989: f64, t64257: f64, t11292: f64, t1156: f64) -> (f64, f64, f64, f64, f64) {
    let t78291 = 0.91082604192152556044e5_f64 * t1164 * t43689 * t78287 * t43692;
    let t78294 = 0.61524113149298439947e4_f64 * t1164 * t64451 * t18622;
    let t78296 = 0.14035736694323150897e2_f64 * t4869 * t21833;
    let t78298 = 12.0_f64 * t64257 * t5989;
    let t78302 = 0.14035736694323150897e2_f64 * t1164 * t11292 * t78287 * t1156;
    (t78291, t78294, t78296, t78298, t78302)
}
