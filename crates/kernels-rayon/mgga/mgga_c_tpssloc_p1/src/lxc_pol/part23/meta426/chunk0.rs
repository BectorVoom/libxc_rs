//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1256/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1256(t1036: f64, t21483: f64, t1041: f64, t13969: f64, t21511: f64, t10413: f64, t10422: f64, t21531: f64, t21486: f64, t3130: f64, t21565: f64, t3070: f64) -> (f64, f64, f64, f64, f64) {
    let t70766 = t21483 * t1036;
    let t70792 = t1041 * t13969 * t21511;
    let t70800 = t10413 * t10422 * t21531;
    let t70805 = t3130 * t13969 * t21486;
    let t70846 = t3070 * t10422 * t21565;
    (t70766, t70792, t70800, t70805, t70846)
}
