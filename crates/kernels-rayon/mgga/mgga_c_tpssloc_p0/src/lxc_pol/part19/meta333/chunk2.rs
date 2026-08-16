//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1196/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1196(t751: f64, t9288: f64, t9897: f64, t2244: f64, t2517: f64, t2658: f64, t39488: f64, t761: f64, t2531: f64, t9919: f64, t707: f64, t9258: f64) -> (f64, f64, f64, f64, f64) {
    let t40726 = t9897 * t751 * t9288;
    let t40727 = 96.0_f64 * t40726;
    let t40729 = t2658 * t2517 * t2244;
    let t40730 = 72.0_f64 * t40729;
    let t40732 = 0.6233709278045326953e3_f64 * t761 * t39488;
    let t40733 = t2531 * t9919;
    let t40734 = 0.14035736694323150897e2_f64 * t40733;
    let t40736 = t707 * t751 * t9258;
    (t40727, t40730, t40732, t40734, t40736)
}
