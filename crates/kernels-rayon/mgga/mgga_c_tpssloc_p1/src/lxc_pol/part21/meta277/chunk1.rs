//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1557/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1557(t761: f64, t9919: f64, t2531: f64, t2535: f64, t2427: f64, t2430: f64, t32: f64, t717: f64, t2244: f64, t751: f64, t2658: f64, t2617: f64, t2629: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9921 = 0.35089341735807877242e1_f64 * t761 * t9919;
    let t9922 = t2531 * t2535;
    let t9924 = t2427 * t2430;
    let t9929 = t32 * t717;
    let t9932 = t751 * t2244;
    let t9933 = t2658 * t9932;
    let t9967 = t2617 * t2629;
    (t9921, t9922, t9924, t9929, t9932, t9933, t9967)
}
